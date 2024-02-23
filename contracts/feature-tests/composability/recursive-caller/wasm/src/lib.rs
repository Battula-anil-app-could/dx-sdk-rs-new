// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            1
// Async Callback:                       1
// Total number of exported functions:   3

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    recursive_caller
    (
        init => init
        recursive_send_funds => recursive_send_funds
    )
}

dharitri_sc_wasm_adapter::async_callback! { recursive_caller }
