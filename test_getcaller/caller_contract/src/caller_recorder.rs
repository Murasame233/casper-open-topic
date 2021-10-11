#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{string::String, vec, vec::Vec};

use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    contracts::NamedKeys, system::CallStackElement, CLType, EntryPoint, EntryPointAccess,
    EntryPoints, Key,
};

const CALLER_KEY: &str = "Caller";

#[no_mangle]
pub extern "C" fn update_caller() {
    let s = runtime::get_call_stack();
    let k = runtime::get_key("Call Stack").unwrap();
    let uref = k.as_uref().unwrap();
    storage::write(*uref, s);
    let caller = runtime::get_caller().to_formatted_string();
    let k = runtime::get_key(CALLER_KEY).unwrap();
    let uref = k.as_uref().unwrap();
    storage::write(*uref, caller);
}

#[no_mangle]
pub extern "C" fn call() {
    let key = Key::URef(storage::new_uref(String::new()));
    runtime::put_key(CALLER_KEY, key);
    let v: Vec<CallStackElement> = vec![];
    let key_call_stack = Key::URef(storage::new_uref(v));
    runtime::put_key("Call Stack", key_call_stack);
    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    let mut entrypoints = EntryPoints::new();
    entrypoints.add_entry_point(EntryPoint::new(
        "update_caller",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    let mut keys = NamedKeys::new();
    keys.insert(CALLER_KEY.into(), key);
    keys.insert("Call Stack".into(), key_call_stack);
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entrypoints, keys);
    runtime::put_key("caller_contract", contract_hash.into());
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key("caller_contract_hash", contract_hash_pack.into());
}
