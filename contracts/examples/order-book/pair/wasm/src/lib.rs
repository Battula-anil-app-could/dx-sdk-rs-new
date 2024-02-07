////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    order_book_pair::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    order_book_pair::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn cancelAllOrders() {
    order_book_pair::endpoints::cancelAllOrders(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn cancelOrders() {
    order_book_pair::endpoints::cancelOrders(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn createBuyOrder() {
    order_book_pair::endpoints::createBuyOrder(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn createSellOrder() {
    order_book_pair::endpoints::createSellOrder(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn freeOrders() {
    order_book_pair::endpoints::freeOrders(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getAddressOrderIds() {
    order_book_pair::endpoints::getAddressOrderIds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getFirstTokenId() {
    order_book_pair::endpoints::getFirstTokenId(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getOrderById() {
    order_book_pair::endpoints::getOrderById(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getOrderIdCounter() {
    order_book_pair::endpoints::getOrderIdCounter(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getSecondTokenId() {
    order_book_pair::endpoints::getSecondTokenId(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn matchOrders() {
    order_book_pair::endpoints::matchOrders(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn startGlobalOperation() {
    order_book_pair::endpoints::startGlobalOperation(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn stopGlobalOperation() {
    order_book_pair::endpoints::stopGlobalOperation(dharitri_wasm_node::vm_api());
}
