////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    lottery_dct::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    lottery_dct::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn buy_ticket() {
    lottery_dct::endpoints::buy_ticket(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn createLotteryPool() {
    lottery_dct::endpoints::createLotteryPool(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn determine_winner() {
    lottery_dct::endpoints::determine_winner(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getLotteryInfo() {
    lottery_dct::endpoints::getLotteryInfo(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn start() {
    lottery_dct::endpoints::start(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn status() {
    lottery_dct::endpoints::status(dharitri_wasm_node::vm_api());
}
