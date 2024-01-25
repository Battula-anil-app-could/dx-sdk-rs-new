////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    child::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    child::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getWrappedMoaxTokenIdentifier() {
    child::endpoints::getWrappedMoaxTokenIdentifier(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn issueWrappedMoax() {
    child::endpoints::issueWrappedMoax(dharitri_wasm_node::vm_api());
}
