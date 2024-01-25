////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    proxy_test_second::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    proxy_test_second::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn messageMe() {
    proxy_test_second::endpoints::messageMe(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payMe() {
    proxy_test_second::endpoints::payMe(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payMeWithResult() {
    proxy_test_second::endpoints::payMeWithResult(dharitri_wasm_node::vm_api());
}
