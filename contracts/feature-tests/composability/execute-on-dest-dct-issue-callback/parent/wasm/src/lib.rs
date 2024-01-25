////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    parent::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    parent::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn deployChildContract() {
    parent::endpoints::deployChildContract(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn deposit() {
    parent::endpoints::deposit(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn executeOnDestIssueToken() {
    parent::endpoints::executeOnDestIssueToken(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getChildContractAddress() {
    parent::endpoints::getChildContractAddress(dharitri_wasm_node::vm_api());
}
