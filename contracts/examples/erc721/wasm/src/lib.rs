////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    erc721::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    erc721::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn approval() {
    erc721::endpoints::approval(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn approve() {
    erc721::endpoints::approve(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mint() {
    erc721::endpoints::mint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn revoke() {
    erc721::endpoints::revoke(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn tokenCount() {
    erc721::endpoints::tokenCount(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn tokenOwner() {
    erc721::endpoints::tokenOwner(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn totalMinted() {
    erc721::endpoints::totalMinted(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transfer() {
    erc721::endpoints::transfer(dharitri_wasm_node::vm_api());
}
