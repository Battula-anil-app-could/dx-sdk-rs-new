// Code generated by the dharitri-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           10
// Async Callback (empty):               1
// Total number of exported functions:  12

#![no_std]

dharitri_wasm_node::wasm_endpoints! {
    ping_pong_moax
    (
        ping
        pong
        pongAll
        getUserAddresses
        getPingAmount
        getDeadline
        getActivationTimestamp
        getMaxFunds
        getUserStatus
        pongAllLastUser
    )
}

dharitri_wasm_node::wasm_empty_callback! {}
