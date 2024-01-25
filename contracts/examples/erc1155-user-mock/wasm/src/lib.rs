////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    erc1155_user_mock::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    erc1155_user_mock::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn onERC1155BatchReceived() {
    erc1155_user_mock::endpoints::onERC1155BatchReceived(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn onERC1155Received() {
    erc1155_user_mock::endpoints::onERC1155Received(dharitri_wasm_node::vm_api());
}
