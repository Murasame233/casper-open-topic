#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ContractHash, RuntimeArgs};

#[no_mangle]
pub extern "C" fn call() {
    let hash = runtime::get_named_arg::<ContractHash>("hash");

    runtime::call_contract(hash, "update_caller", runtime_args! {})
}
