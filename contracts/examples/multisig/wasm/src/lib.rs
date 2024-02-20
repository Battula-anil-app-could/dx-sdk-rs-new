// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           20
// Async Callback:                       1
// Total number of exported functions:  22

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    multisig
    (
        init => init
        deposit => deposit
        signed => signed
        sign => sign
        unsign => unsign
        discardAction => discard_action
        getQuorum => quorum
        getNumBoardMembers => num_board_members
        getNumProposers => num_proposers
        getActionLastIndex => get_action_last_index
        proposeAddBoardMember => propose_add_board_member
        proposeAddProposer => propose_add_proposer
        proposeRemoveUser => propose_remove_user
        proposeChangeQuorum => propose_change_quorum
        proposeTransferExecute => propose_transfer_execute
        proposeAsyncCall => propose_async_call
        proposeSCDeployFromSource => propose_sc_deploy_from_source
        proposeSCUpgradeFromSource => propose_sc_upgrade_from_source
        quorumReached => quorum_reached
        performAction => perform_action_endpoint
        dnsRegister => dns_register
    )
}

dharitri_sc_wasm_adapter::async_callback! { multisig }
