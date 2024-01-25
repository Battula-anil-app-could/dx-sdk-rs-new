////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    moax_dct_swap::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    moax_dct_swap::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getLockedMoaxBalance() {
    moax_dct_swap::endpoints::getLockedMoaxBalance(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getUnusedWrappedMoax() {
    moax_dct_swap::endpoints::getUnusedWrappedMoax(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getWrappedMoaxTokenIdentifier() {
    moax_dct_swap::endpoints::getWrappedMoaxTokenIdentifier(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn issueWrappedMoax() {
    moax_dct_swap::endpoints::issueWrappedMoax(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mintWrappedMoax() {
    moax_dct_swap::endpoints::mintWrappedMoax(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn unwrapMoax() {
    moax_dct_swap::endpoints::unwrapMoax(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn wrapMoax() {
    moax_dct_swap::endpoints::wrapMoax(dharitri_wasm_node::vm_api());
}
