#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{string::String, vec};
use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLType, CLTyped, ContractHash, EntryPoint,
    EntryPointAccess, EntryPoints, Parameter, RuntimeArgs,
};

#[no_mangle]
pub extern "C" fn cal() {
    let hash = runtime::get_named_arg::<ContractHash>("hash");

    runtime::call_contract(hash, "update_caller", runtime_args! {})
}
#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    let mut entrypoints = EntryPoints::new();
    entrypoints.add_entry_point(EntryPoint::new(
        "cal",
        vec![Parameter::new("hash", String::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    let keys = NamedKeys::new();
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entrypoints, keys);
    runtime::put_key("cal_contract", contract_hash.into());
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key("cal_contract_hash", contract_hash_pack.into());
}
