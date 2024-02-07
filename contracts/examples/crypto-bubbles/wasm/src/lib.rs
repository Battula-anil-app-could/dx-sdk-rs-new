////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    crypto_bubbles::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    crypto_bubbles::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn balanceOf() {
    crypto_bubbles::endpoints::balanceOf(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn joinGame() {
    crypto_bubbles::endpoints::joinGame(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn rewardAndSendToWallet() {
    crypto_bubbles::endpoints::rewardAndSendToWallet(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn rewardWinner() {
    crypto_bubbles::endpoints::rewardWinner(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn topUp() {
    crypto_bubbles::endpoints::topUp(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn withdraw() {
    crypto_bubbles::endpoints::withdraw(dharitri_wasm_node::vm_api());
}
