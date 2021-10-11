#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use casper_contract::contract_api::{
    runtime::{self, call_contract},
    storage,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLType, ContractHash, EntryPoint, EntryPoints, Key,
    Parameter, RuntimeArgs,
};

#[no_mangle]
pub extern "C" fn callback() {
    let log_uref = runtime::get_key("log").unwrap().into_uref().unwrap();
    let mut log: Vec<(String, u32)> = storage::read(log_uref).unwrap().unwrap();

    let caller: String = runtime::get_named_arg("caller");
    let count: u32 = runtime::get_named_arg("count");

    log.push((caller, count));

    storage::write(log_uref, log);
}

#[no_mangle]
pub extern "C" fn call() {
    // get counter contract hash
    let contract_hash_string: String = runtime::get_named_arg("hash");
    let counter_hash = ContractHash::from_formatted_str(&contract_hash_string).unwrap();

    let mut keys = NamedKeys::new();
    // store log to user context
    let log: Vec<(String, u32)> = vec![];
    let log_uref = storage::new_uref(log);
    runtime::put_key("log", Key::URef(log_uref));
    keys.insert("log".into(), Key::URef(log_uref));

    let mut entry = EntryPoints::new();
    entry.add_entry_point(EntryPoint::new(
        "callback",
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("caller", CLType::String),
            Parameter::new("count", CLType::U32),
        ],
        casper_types::CLType::Unit,
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));

    let (pack_hash, _) = storage::create_contract_package_at_hash();
    let (hash, _) = storage::add_contract_version(pack_hash, entry, keys);
    let uref = storage::new_uref(hash);

    // set callback
    runtime::put_key("counter_hash", Key::URef(uref));
    call_contract(
        counter_hash,
        "set_event_callback",
        runtime_args! {
            "event" => "count".to_string(),
            "call_back_contract_hash" => hash.to_formatted_string(),
            "call_back_contract_entry" => "callback".to_string()
        },
    )
}
