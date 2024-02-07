////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    erc1155::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    erc1155::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn balanceOf() {
    erc1155::endpoints::balanceOf(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn balanceOfBatch() {
    erc1155::endpoints::balanceOfBatch(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn burn() {
    erc1155::endpoints::burn(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn createToken() {
    erc1155::endpoints::createToken(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getTokenOwner() {
    erc1155::endpoints::getTokenOwner(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getTokenTypeCreator() {
    erc1155::endpoints::getTokenTypeCreator(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getTokenTypeUri() {
    erc1155::endpoints::getTokenTypeUri(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn isApprovedForAll() {
    erc1155::endpoints::isApprovedForAll(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn isFungible() {
    erc1155::endpoints::isFungible(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mint() {
    erc1155::endpoints::mint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn safeBatchTransferFrom() {
    erc1155::endpoints::safeBatchTransferFrom(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn safeTransferFrom() {
    erc1155::endpoints::safeTransferFrom(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setApprovalForAll() {
    erc1155::endpoints::setApprovalForAll(dharitri_wasm_node::vm_api());
}
