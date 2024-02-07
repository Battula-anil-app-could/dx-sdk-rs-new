////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    set_repeat::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    set_repeat::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add() {
    set_repeat::endpoints::add(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn count() {
    set_repeat::endpoints::count(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getBenchmark() {
    set_repeat::endpoints::getBenchmark(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn remove() {
    set_repeat::endpoints::remove(dharitri_wasm_node::vm_api());
}
