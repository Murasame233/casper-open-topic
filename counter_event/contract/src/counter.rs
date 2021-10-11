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
    runtime::{self, call_contract, revert},
    storage,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, ApiError, CLType, ContractHash, EntryPoint, EntryPoints,
    Key, Parameter, RuntimeArgs,
};
#[repr(u16)]
enum CounterError {
    InvalidEventName = 0,
}
impl From<CounterError> for ApiError {
    fn from(error: CounterError) -> Self {
        ApiError::User(error as u16)
    }
}

fn callback() {
    // get callbacks
    let event_callbacks: Vec<(String, ContractHash, String)> =
        storage::read(runtime::get_key("callbacks").unwrap().into_uref().unwrap())
            .unwrap()
            .unwrap();
    // get count now and caller
    let count: u32 = storage::read(runtime::get_key("count").unwrap().into_uref().unwrap())
        .unwrap()
        .unwrap();
    let caller = runtime::get_caller().to_formatted_string();

    // run callback
    event_callbacks
        .iter()
        .map(|f| {
            call_contract::<()>(
                f.1,
                &f.0,
                runtime_args! {
                    "name" => "count".to_string(),
                    "caller" => caller.clone(),
                    "count" => count
                },
            );
        })
        .count();
}

#[no_mangle]
pub extern "C" fn count() {
    let uref = runtime::get_key("count").unwrap().into_uref().unwrap();
    let mut count: u32 = storage::read(uref).unwrap().unwrap();
    count += 1;
    storage::write(uref, count);
    callback();
}

// This entry will set event callback
#[no_mangle]
pub extern "C" fn set_event_callback() {
    // get args
    let event: String = runtime::get_named_arg("event");
    let contract_string: String = runtime::get_named_arg("call_back_contract_hash");
    let contract_hash = ContractHash::from_formatted_str(&contract_string).unwrap();
    let contract_entry: String = runtime::get_named_arg("call_back_contract_entry");

    // valid event name
    if event != "count" {
        revert(CounterError::InvalidEventName);
    }

    // update callbacks
    let mut event_callbacks: Vec<(String, ContractHash, String)> =
        storage::read(runtime::get_key("callbacks").unwrap().into_uref().unwrap())
            .unwrap()
            .unwrap();

    event_callbacks.push((contract_entry, contract_hash, event));

    storage::write(
        runtime::get_key("callbacks").unwrap().into_uref().unwrap(),
        event_callbacks,
    );
}
#[no_mangle]
pub extern "C" fn call() {
    let mut keys = NamedKeys::new();

    let counter = storage::new_uref(0u32);
    runtime::put_key("count", Key::URef(counter));
    keys.insert("count".into(), Key::URef(counter));

    let event_callbacks: Vec<(String, ContractHash, String)> = vec![];
    let counter = storage::new_uref(event_callbacks);
    runtime::put_key("callbacks", Key::URef(counter));
    keys.insert("callbacks".into(), Key::URef(counter));

    let mut entry = EntryPoints::new();
    entry.add_entry_point(EntryPoint::new(
        "count",
        vec![],
        casper_types::CLType::Unit,
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    entry.add_entry_point(EntryPoint::new(
        "set_event_callback",
        vec![
            Parameter::new("call_back_contract_hash", CLType::String),
            Parameter::new("call_back_contract_entry", CLType::String),
            Parameter::new("event", CLType::String),
        ],
        casper_types::CLType::Unit,
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    let (pack_hash, _) = storage::create_contract_package_at_hash();
    let (hash, _) = storage::add_contract_version(pack_hash, entry, keys);
    let uref = storage::new_uref(hash);
    runtime::put_key("counter_hash", Key::URef(uref));
}
