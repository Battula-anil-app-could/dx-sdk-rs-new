////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    order_book_factory::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    order_book_factory::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn createPair() {
    order_book_factory::endpoints::createPair(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getPair() {
    order_book_factory::endpoints::getPair(dharitri_wasm_node::vm_api());
}
