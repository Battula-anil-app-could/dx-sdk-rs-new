////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    factorial::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    factorial::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn factorial() {
    factorial::endpoints::factorial(dharitri_wasm_node::vm_api());
}
