////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    forwarder_raw::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    forwarder_raw::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_execute_on_dest_context() {
    forwarder_raw::endpoints::call_execute_on_dest_context(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_execute_on_dest_context_by_caller() {
    forwarder_raw::endpoints::call_execute_on_dest_context_by_caller(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_execute_on_dest_context_readonly() {
    forwarder_raw::endpoints::call_execute_on_dest_context_readonly(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_execute_on_dest_context_twice() {
    forwarder_raw::endpoints::call_execute_on_dest_context_twice(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_execute_on_same_context() {
    forwarder_raw::endpoints::call_execute_on_same_context(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callback_data() {
    forwarder_raw::endpoints::callback_data(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn clear_callback_info() {
    forwarder_raw::endpoints::clear_callback_info(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn deploy_contract() {
    forwarder_raw::endpoints::deploy_contract(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn deploy_from_source() {
    forwarder_raw::endpoints::deploy_from_source(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_async_call() {
    forwarder_raw::endpoints::forward_async_call(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_async_call_half_payment() {
    forwarder_raw::endpoints::forward_async_call_half_payment(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_async_retrieve_multi_transfer_funds() {
    forwarder_raw::endpoints::forward_async_retrieve_multi_transfer_funds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_direct_dct_via_transf_exec() {
    forwarder_raw::endpoints::forward_direct_dct_via_transf_exec(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_payment() {
    forwarder_raw::endpoints::forward_payment(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_transf_exec() {
    forwarder_raw::endpoints::forward_transf_exec(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_transf_exec_moax() {
    forwarder_raw::endpoints::forward_transf_exec_moax(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_transf_exec_dct() {
    forwarder_raw::endpoints::forward_transf_exec_dct(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forwarder_async_send_and_retrieve_multi_transfer_funds() {
    forwarder_raw::endpoints::forwarder_async_send_and_retrieve_multi_transfer_funds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn upgrade() {
    forwarder_raw::endpoints::upgrade(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn upgrade_from_source() {
    forwarder_raw::endpoints::upgrade_from_source(dharitri_wasm_node::vm_api());
}
