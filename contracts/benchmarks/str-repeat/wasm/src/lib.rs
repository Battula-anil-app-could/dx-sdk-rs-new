////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    str_repeat::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    str_repeat::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getByteArray() {
    str_repeat::endpoints::getByteArray(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getByteArrayLength() {
    str_repeat::endpoints::getByteArrayLength(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn repeat() {
    str_repeat::endpoints::repeat(dharitri_wasm_node::vm_api());
}
