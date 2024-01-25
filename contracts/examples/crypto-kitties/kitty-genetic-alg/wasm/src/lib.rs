////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    kitty_genetic_alg::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    kitty_genetic_alg::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn generateKittyGenes() {
    kitty_genetic_alg::endpoints::generateKittyGenes(dharitri_wasm_node::vm_api());
}
