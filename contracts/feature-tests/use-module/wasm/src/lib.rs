////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    use_module::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    use_module::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_mod_a() {
    use_module::endpoints::call_mod_a(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_mod_b() {
    use_module::endpoints::call_mod_b(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_mod_c() {
    use_module::endpoints::call_mod_c(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn cancel() {
    use_module::endpoints::cancel(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn changeLockTimeAfterVotingEndsInBlocks() {
    use_module::endpoints::changeLockTimeAfterVotingEndsInBlocks(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn changeMaxActionsPerProposal() {
    use_module::endpoints::changeMaxActionsPerProposal(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn changeMinTokenBalanceForProposing() {
    use_module::endpoints::changeMinTokenBalanceForProposing(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn changeQuorum() {
    use_module::endpoints::changeQuorum(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn changeVotingDelayInBlocks() {
    use_module::endpoints::changeVotingDelayInBlocks(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn changeVotingPeriodInBlocks() {
    use_module::endpoints::changeVotingPeriodInBlocks(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn checkFeatureGuard() {
    use_module::endpoints::checkFeatureGuard(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn checkPause() {
    use_module::endpoints::checkPause(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn depositTokensForAction() {
    use_module::endpoints::depositTokensForAction(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn dnsRegister() {
    use_module::endpoints::dnsRegister(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn downvote() {
    use_module::endpoints::downvote(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn execute() {
    use_module::endpoints::execute(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getGovernanceTokenId() {
    use_module::endpoints::getGovernanceTokenId(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getLockTimeAfterVotingEndsInBlocks() {
    use_module::endpoints::getLockTimeAfterVotingEndsInBlocks(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getMaxActionsPerProposal() {
    use_module::endpoints::getMaxActionsPerProposal(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getMinTokenBalanceForProposing() {
    use_module::endpoints::getMinTokenBalanceForProposing(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getProposalActions() {
    use_module::endpoints::getProposalActions(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getProposalDescription() {
    use_module::endpoints::getProposalDescription(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getProposalStatus() {
    use_module::endpoints::getProposalStatus(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getProposer() {
    use_module::endpoints::getProposer(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getQuorum() {
    use_module::endpoints::getQuorum(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getTotalDownvotes() {
    use_module::endpoints::getTotalDownvotes(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getTotalVotes() {
    use_module::endpoints::getTotalVotes(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getVotingDelayInBlocks() {
    use_module::endpoints::getVotingDelayInBlocks(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getVotingPeriodInBlocks() {
    use_module::endpoints::getVotingPeriodInBlocks(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn initGovernanceModule() {
    use_module::endpoints::initGovernanceModule(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn isPaused() {
    use_module::endpoints::isPaused(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn issueToken() {
    use_module::endpoints::issueToken(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn pause() {
    use_module::endpoints::pause(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn propose() {
    use_module::endpoints::propose(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn queue() {
    use_module::endpoints::queue(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setFeatureFlag() {
    use_module::endpoints::setFeatureFlag(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setLocalRoles() {
    use_module::endpoints::setLocalRoles(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn unpause() {
    use_module::endpoints::unpause(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn vote() {
    use_module::endpoints::vote(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn withdrawGovernanceTokens() {
    use_module::endpoints::withdrawGovernanceTokens(dharitri_wasm_node::vm_api());
}
