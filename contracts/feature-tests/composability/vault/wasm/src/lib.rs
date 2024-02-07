////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    vault::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    vault::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn accept_funds() {
    vault::endpoints::accept_funds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn accept_funds_echo_payment() {
    vault::endpoints::accept_funds_echo_payment(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn accept_funds_multi_transfer() {
    vault::endpoints::accept_funds_multi_transfer(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn accept_multi_funds_echo() {
    vault::endpoints::accept_multi_funds_echo(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn burn_and_create_retrive_async() {
    vault::endpoints::burn_and_create_retrive_async(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_counts() {
    vault::endpoints::call_counts(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_arguments() {
    vault::endpoints::echo_arguments(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_caller() {
    vault::endpoints::echo_caller(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_owner_address() {
    vault::endpoints::get_owner_address(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn just_accept_funds() {
    vault::endpoints::just_accept_funds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn reject_funds() {
    vault::endpoints::reject_funds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn retrieve_funds() {
    vault::endpoints::retrieve_funds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn retrieve_multi_funds_async() {
    vault::endpoints::retrieve_multi_funds_async(dharitri_wasm_node::vm_api());
}
