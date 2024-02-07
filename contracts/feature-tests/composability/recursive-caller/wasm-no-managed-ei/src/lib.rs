////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    recursive_caller::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    recursive_caller::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn recursive_send_funds() {
    recursive_caller::endpoints::recursive_send_funds(dharitri_wasm_node::vm_api());
}
