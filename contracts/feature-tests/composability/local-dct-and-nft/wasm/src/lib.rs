////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    local_dct_and_nft::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    local_dct_and_nft::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCurrentNftNonce() {
    local_dct_and_nft::endpoints::getCurrentNftNonce(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getFungibleDctBalance() {
    local_dct_and_nft::endpoints::getFungibleDctBalance(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getNftBalance() {
    local_dct_and_nft::endpoints::getNftBalance(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn issueFungibleToken() {
    local_dct_and_nft::endpoints::issueFungibleToken(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn lastErrorMessage() {
    local_dct_and_nft::endpoints::lastErrorMessage(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn lastIssuedToken() {
    local_dct_and_nft::endpoints::lastIssuedToken(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn localBurn() {
    local_dct_and_nft::endpoints::localBurn(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn localMint() {
    local_dct_and_nft::endpoints::localMint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn nftAddQuantity() {
    local_dct_and_nft::endpoints::nftAddQuantity(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn nftBurn() {
    local_dct_and_nft::endpoints::nftBurn(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn nftCreate() {
    local_dct_and_nft::endpoints::nftCreate(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn nftIssue() {
    local_dct_and_nft::endpoints::nftIssue(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setLocalRoles() {
    local_dct_and_nft::endpoints::setLocalRoles(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sftIssue() {
    local_dct_and_nft::endpoints::sftIssue(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferNftViaAsyncCall() {
    local_dct_and_nft::endpoints::transferNftViaAsyncCall(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transfer_nft_and_execute() {
    local_dct_and_nft::endpoints::transfer_nft_and_execute(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn unsetLocalRoles() {
    local_dct_and_nft::endpoints::unsetLocalRoles(dharitri_wasm_node::vm_api());
}
