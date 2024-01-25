////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    abi_tester::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    abi_tester::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn address_vs_h256() {
    abi_tester::endpoints::address_vs_h256(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_abi_test_type() {
    abi_tester::endpoints::echo_abi_test_type(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_enum() {
    abi_tester::endpoints::echo_enum(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn dct_local_role() {
    abi_tester::endpoints::dct_local_role(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn dct_token_data() {
    abi_tester::endpoints::dct_token_data(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn dct_token_payment() {
    abi_tester::endpoints::dct_token_payment(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn managed_address_vs_byte_array() {
    abi_tester::endpoints::managed_address_vs_byte_array(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn multi_result_3() {
    abi_tester::endpoints::multi_result_3(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn multi_result_4() {
    abi_tester::endpoints::multi_result_4(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn multi_result_vec() {
    abi_tester::endpoints::multi_result_vec(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn optional_arg() {
    abi_tester::endpoints::optional_arg(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn optional_result() {
    abi_tester::endpoints::optional_result(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_any_token() {
    abi_tester::endpoints::payable_any_token(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_moax() {
    abi_tester::endpoints::payable_moax(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_some_token() {
    abi_tester::endpoints::payable_some_token(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sample_storage_mapper() {
    abi_tester::endpoints::sample_storage_mapper(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn var_args() {
    abi_tester::endpoints::var_args(dharitri_wasm_node::vm_api());
}
