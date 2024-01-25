////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    send_tx_repeat::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    send_tx_repeat::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn repeat() {
    send_tx_repeat::endpoints::repeat(dharitri_wasm_node::vm_api());
}
