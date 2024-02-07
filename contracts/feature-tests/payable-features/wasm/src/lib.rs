////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    payable_features::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    payable_features::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_call_value() {
    payable_features::endpoints::echo_call_value(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_any_1() {
    payable_features::endpoints::payable_any_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_any_2() {
    payable_features::endpoints::payable_any_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_any_3() {
    payable_features::endpoints::payable_any_3(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_any_4() {
    payable_features::endpoints::payable_any_4(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_moax_1() {
    payable_features::endpoints::payable_moax_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_moax_2() {
    payable_features::endpoints::payable_moax_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_moax_3() {
    payable_features::endpoints::payable_moax_3(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_moax_4() {
    payable_features::endpoints::payable_moax_4(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_token_1() {
    payable_features::endpoints::payable_token_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_token_2() {
    payable_features::endpoints::payable_token_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_token_3() {
    payable_features::endpoints::payable_token_3(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_token_4() {
    payable_features::endpoints::payable_token_4(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payment_multiple() {
    payable_features::endpoints::payment_multiple(dharitri_wasm_node::vm_api());
}
