////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    second_contract::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    second_contract::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn acceptDctPayment() {
    second_contract::endpoints::acceptDctPayment(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getdctTokenName() {
    second_contract::endpoints::getdctTokenName(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn rejectDctPayment() {
    second_contract::endpoints::rejectDctPayment(dharitri_wasm_node::vm_api());
}
