////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    vec_repeat::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    vec_repeat::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add() {
    vec_repeat::endpoints::add(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn count() {
    vec_repeat::endpoints::count(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getBenchmark() {
    vec_repeat::endpoints::getBenchmark(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn remove() {
    vec_repeat::endpoints::remove(dharitri_wasm_node::vm_api());
}
