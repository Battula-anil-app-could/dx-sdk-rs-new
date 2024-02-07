////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    bonding_curve_contract::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    bonding_curve_contract::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn buyToken() {
    bonding_curve_contract::endpoints::buyToken(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn claim() {
    bonding_curve_contract::endpoints::claim(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn deposit() {
    bonding_curve_contract::endpoints::deposit(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getTokenAvailability() {
    bonding_curve_contract::endpoints::getTokenAvailability(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_buy_price() {
    bonding_curve_contract::endpoints::get_buy_price(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_sell_price() {
    bonding_curve_contract::endpoints::get_sell_price(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sellToken() {
    bonding_curve_contract::endpoints::sellToken(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setBondingCurve() {
    bonding_curve_contract::endpoints::setBondingCurve(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setLocalRoles() {
    bonding_curve_contract::endpoints::setLocalRoles(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn unsetLocalRoles() {
    bonding_curve_contract::endpoints::unsetLocalRoles(dharitri_wasm_node::vm_api());
}
