////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    linked_list_repeat::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    linked_list_repeat::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add() {
    linked_list_repeat::endpoints::add(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn count() {
    linked_list_repeat::endpoints::count(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getBenchmark() {
    linked_list_repeat::endpoints::getBenchmark(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn remove() {
    linked_list_repeat::endpoints::remove(dharitri_wasm_node::vm_api());
}
