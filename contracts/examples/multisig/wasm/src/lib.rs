////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    multisig::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    multisig::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn deposit() {
    multisig::endpoints::deposit(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn discardAction() {
    multisig::endpoints::discardAction(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getActionData() {
    multisig::endpoints::getActionData(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getActionLastIndex() {
    multisig::endpoints::getActionLastIndex(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getActionSignerCount() {
    multisig::endpoints::getActionSignerCount(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getActionSigners() {
    multisig::endpoints::getActionSigners(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getActionValidSignerCount() {
    multisig::endpoints::getActionValidSignerCount(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getAllBoardMembers() {
    multisig::endpoints::getAllBoardMembers(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getAllProposers() {
    multisig::endpoints::getAllProposers(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getNumBoardMembers() {
    multisig::endpoints::getNumBoardMembers(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getNumProposers() {
    multisig::endpoints::getNumProposers(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getPendingActionFullInfo() {
    multisig::endpoints::getPendingActionFullInfo(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getQuorum() {
    multisig::endpoints::getQuorum(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn performAction() {
    multisig::endpoints::performAction(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn proposeAddBoardMember() {
    multisig::endpoints::proposeAddBoardMember(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn proposeAddProposer() {
    multisig::endpoints::proposeAddProposer(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn proposeChangeQuorum() {
    multisig::endpoints::proposeChangeQuorum(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn proposeRemoveUser() {
    multisig::endpoints::proposeRemoveUser(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn proposeSCDeploy() {
    multisig::endpoints::proposeSCDeploy(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn proposeSCDeployFromSource() {
    multisig::endpoints::proposeSCDeployFromSource(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn proposeSCUpgrade() {
    multisig::endpoints::proposeSCUpgrade(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn proposeSCUpgradeFromSource() {
    multisig::endpoints::proposeSCUpgradeFromSource(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn proposeSendMoax() {
    multisig::endpoints::proposeSendMoax(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn proposeSendDct() {
    multisig::endpoints::proposeSendDct(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn quorumReached() {
    multisig::endpoints::quorumReached(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sign() {
    multisig::endpoints::sign(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn signed() {
    multisig::endpoints::signed(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn unsign() {
    multisig::endpoints::unsign(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn userRole() {
    multisig::endpoints::userRole(dharitri_wasm_node::vm_api());
}
