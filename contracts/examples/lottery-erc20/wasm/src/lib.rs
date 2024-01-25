////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    lottery_erc20::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    lottery_erc20::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn buy_ticket() {
    lottery_erc20::endpoints::buy_ticket(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn createLotteryPool() {
    lottery_erc20::endpoints::createLotteryPool(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn determine_winner() {
    lottery_erc20::endpoints::determine_winner(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn erc20ContractManagedAddress() {
    lottery_erc20::endpoints::erc20ContractManagedAddress(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn lotteryInfo() {
    lottery_erc20::endpoints::lotteryInfo(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn start() {
    lottery_erc20::endpoints::start(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn status() {
    lottery_erc20::endpoints::status(dharitri_wasm_node::vm_api());
}
