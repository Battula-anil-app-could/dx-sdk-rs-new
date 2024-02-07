////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    ping_pong_moax::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    ping_pong_moax::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getActivationTimestamp() {
    ping_pong_moax::endpoints::getActivationTimestamp(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getDeadline() {
    ping_pong_moax::endpoints::getDeadline(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getMaxFunds() {
    ping_pong_moax::endpoints::getMaxFunds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getPingAmount() {
    ping_pong_moax::endpoints::getPingAmount(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getUserAddresses() {
    ping_pong_moax::endpoints::getUserAddresses(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getUserStatus() {
    ping_pong_moax::endpoints::getUserStatus(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn ping() {
    ping_pong_moax::endpoints::ping(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn pong() {
    ping_pong_moax::endpoints::pong(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn pongAll() {
    ping_pong_moax::endpoints::pongAll(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn pongAllLastUser() {
    ping_pong_moax::endpoints::pongAllLastUser(dharitri_wasm_node::vm_api());
}
