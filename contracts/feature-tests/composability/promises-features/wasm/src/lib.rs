// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback (empty):               1
// Promise callbacks:                    2
// Total number of exported functions:  11

#![no_std]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    promises_features
    (
        init => init
        forward_promise_accept_funds => forward_promise_accept_funds
        forward_promise_retrieve_funds => forward_promise_retrieve_funds
        callback_data => callback_data
        callback_data_at_index => callback_data_at_index
        clear_callback_data => clear_callback_data
        promise_raw_single_token => promise_raw_single_token
        promise_raw_multi_transfer => promise_raw_multi_transfer
        retrieve_funds_callback => retrieve_funds_callback
        the_one_callback => the_one_callback
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
