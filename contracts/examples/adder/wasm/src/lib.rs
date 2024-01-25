////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    adder::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    adder::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add() {
    adder::endpoints::add(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getSum() {
    adder::endpoints::getSum(dharitri_wasm_node::vm_api());
}
