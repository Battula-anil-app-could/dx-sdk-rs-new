////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    first_contract::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    first_contract::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getSecondContractAddress() {
    first_contract::endpoints::getSecondContractAddress(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getdctTokenName() {
    first_contract::endpoints::getdctTokenName(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferToSecondContractFull() {
    first_contract::endpoints::transferToSecondContractFull(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferToSecondContractFullWithTransferAndExecute() {
    first_contract::endpoints::transferToSecondContractFullWithTransferAndExecute(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferToSecondContractHalf() {
    first_contract::endpoints::transferToSecondContractHalf(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferToSecondContractRejected() {
    first_contract::endpoints::transferToSecondContractRejected(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferToSecondContractRejectedWithTransferAndExecute() {
    first_contract::endpoints::transferToSecondContractRejectedWithTransferAndExecute(dharitri_wasm_node::vm_api());
}
