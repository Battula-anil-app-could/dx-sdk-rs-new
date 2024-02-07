////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    panic_message_features::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    panic_message_features::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn panicWithMessage() {
    panic_message_features::endpoints::panicWithMessage(dharitri_wasm_node::vm_api());
}
