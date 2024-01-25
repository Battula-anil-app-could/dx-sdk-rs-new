////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    map_repeat::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    map_repeat::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add() {
    map_repeat::endpoints::add(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn count() {
    map_repeat::endpoints::count(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn remove() {
    map_repeat::endpoints::remove(dharitri_wasm_node::vm_api());
}
