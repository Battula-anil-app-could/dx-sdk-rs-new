////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    forwarder::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    forwarder::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn buy_nft() {
    forwarder::endpoints::buy_nft(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callback_data() {
    forwarder::endpoints::callback_data(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callback_data_at_index() {
    forwarder::endpoints::callback_data_at_index(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn changeOwnerAddress() {
    forwarder::endpoints::changeOwnerAddress(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn clear_callback_data() {
    forwarder::endpoints::clear_callback_data(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn create_and_send() {
    forwarder::endpoints::create_and_send(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn deploy_contract() {
    forwarder::endpoints::deploy_contract(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn deploy_two_contracts() {
    forwarder::endpoints::deploy_two_contracts(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn deploy_vault_from_source() {
    forwarder::endpoints::deploy_vault_from_source(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_arguments_sync() {
    forwarder::endpoints::echo_arguments_sync(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_arguments_sync_range() {
    forwarder::endpoints::echo_arguments_sync_range(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_arguments_sync_twice() {
    forwarder::endpoints::echo_arguments_sync_twice(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_async_accept_funds() {
    forwarder::endpoints::forward_async_accept_funds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_async_accept_funds_half_payment() {
    forwarder::endpoints::forward_async_accept_funds_half_payment(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_async_accept_funds_with_fees() {
    forwarder::endpoints::forward_async_accept_funds_with_fees(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_async_retrieve_funds() {
    forwarder::endpoints::forward_async_retrieve_funds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_sync_accept_funds() {
    forwarder::endpoints::forward_sync_accept_funds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_sync_accept_funds_multi_transfer() {
    forwarder::endpoints::forward_sync_accept_funds_multi_transfer(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_sync_accept_funds_then_read() {
    forwarder::endpoints::forward_sync_accept_funds_then_read(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_sync_accept_funds_with_fees() {
    forwarder::endpoints::forward_sync_accept_funds_with_fees(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_sync_retrieve_funds() {
    forwarder::endpoints::forward_sync_retrieve_funds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_transf_exec_accept_funds() {
    forwarder::endpoints::forward_transf_exec_accept_funds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_transf_exec_accept_funds_multi_transfer() {
    forwarder::endpoints::forward_transf_exec_accept_funds_multi_transfer(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_transf_exec_accept_funds_return_values() {
    forwarder::endpoints::forward_transf_exec_accept_funds_return_values(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_transf_exec_accept_funds_twice() {
    forwarder::endpoints::forward_transf_exec_accept_funds_twice(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_transf_execu_accept_funds_with_fees() {
    forwarder::endpoints::forward_transf_execu_accept_funds_with_fees(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCurrentNftNonce() {
    forwarder::endpoints::getCurrentNftNonce(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getFungibleDctBalance() {
    forwarder::endpoints::getFungibleDctBalance(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_nft_balance() {
    forwarder::endpoints::get_nft_balance(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn issue_fungible_token() {
    forwarder::endpoints::issue_fungible_token(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn lastErrorMessage() {
    forwarder::endpoints::lastErrorMessage(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn lastIssuedToken() {
    forwarder::endpoints::lastIssuedToken(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn local_burn() {
    forwarder::endpoints::local_burn(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn local_mint() {
    forwarder::endpoints::local_mint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn multi_transfer_via_async() {
    forwarder::endpoints::multi_transfer_via_async(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn nft_add_quantity() {
    forwarder::endpoints::nft_add_quantity(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn nft_burn() {
    forwarder::endpoints::nft_burn(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn nft_create() {
    forwarder::endpoints::nft_create(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn nft_decode_complex_attributes() {
    forwarder::endpoints::nft_decode_complex_attributes(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn nft_issue() {
    forwarder::endpoints::nft_issue(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn send_moax() {
    forwarder::endpoints::send_moax(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn send_dct() {
    forwarder::endpoints::send_dct(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn send_dct_direct_multi_transfer() {
    forwarder::endpoints::send_dct_direct_multi_transfer(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn send_dct_twice() {
    forwarder::endpoints::send_dct_twice(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn send_dct_with_fees() {
    forwarder::endpoints::send_dct_with_fees(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn send_funds_twice() {
    forwarder::endpoints::send_funds_twice(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setLocalRoles() {
    forwarder::endpoints::setLocalRoles(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sft_issue() {
    forwarder::endpoints::sft_issue(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transfer_nft_and_execute() {
    forwarder::endpoints::transfer_nft_and_execute(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transfer_nft_via_async_call() {
    forwarder::endpoints::transfer_nft_via_async_call(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn unsetLocalRoles() {
    forwarder::endpoints::unsetLocalRoles(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn upgradeVault() {
    forwarder::endpoints::upgradeVault(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn upgrade_vault_from_source() {
    forwarder::endpoints::upgrade_vault_from_source(dharitri_wasm_node::vm_api());
}
