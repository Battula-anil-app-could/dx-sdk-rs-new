////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    queue_repeat::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    queue_repeat::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add() {
    queue_repeat::endpoints::add(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn count() {
    queue_repeat::endpoints::count(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getBenchmark() {
    queue_repeat::endpoints::getBenchmark(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn remove() {
    queue_repeat::endpoints::remove(dharitri_wasm_node::vm_api());
}
