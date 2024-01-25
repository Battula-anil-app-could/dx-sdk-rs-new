////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    kitty_ownership::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    kitty_ownership::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn allowAuctioning() {
    kitty_ownership::endpoints::allowAuctioning(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn approve() {
    kitty_ownership::endpoints::approve(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn approveSiring() {
    kitty_ownership::endpoints::approveSiring(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn approveSiringAndReturnKitty() {
    kitty_ownership::endpoints::approveSiringAndReturnKitty(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn balanceOf() {
    kitty_ownership::endpoints::balanceOf(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn birthFee() {
    kitty_ownership::endpoints::birthFee(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn breedWith() {
    kitty_ownership::endpoints::breedWith(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn canBreedWith() {
    kitty_ownership::endpoints::canBreedWith(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn claim() {
    kitty_ownership::endpoints::claim(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn createGenZeroKitty() {
    kitty_ownership::endpoints::createGenZeroKitty(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getKittyById() {
    kitty_ownership::endpoints::getKittyById(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn giveBirth() {
    kitty_ownership::endpoints::giveBirth(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn isPregnant() {
    kitty_ownership::endpoints::isPregnant(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn isReadyToBreed() {
    kitty_ownership::endpoints::isReadyToBreed(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn ownerOf() {
    kitty_ownership::endpoints::ownerOf(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setGeneScienceContractAddress() {
    kitty_ownership::endpoints::setGeneScienceContractAddress(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setKittyAuctionContractAddress() {
    kitty_ownership::endpoints::setKittyAuctionContractAddress(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn tokensOfOwner() {
    kitty_ownership::endpoints::tokensOfOwner(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn totalSupply() {
    kitty_ownership::endpoints::totalSupply(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transfer() {
    kitty_ownership::endpoints::transfer(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transfer_from() {
    kitty_ownership::endpoints::transfer_from(dharitri_wasm_node::vm_api());
}
