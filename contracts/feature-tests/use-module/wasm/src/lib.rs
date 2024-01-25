////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

dharitri_wasm_node::wasm_endpoints! {
   use_module
   (
        init
        callBack
        call_mod_a
        call_mod_b
        call_mod_c
        cancel
        changeLockTimeAfterVotingEndsInBlocks
        changeMaxActionsPerProposal
        changeMinTokenBalanceForProposing
        changeQuorum
        changeVotingDelayInBlocks
        changeVotingPeriodInBlocks
        checkFeatureGuard
        checkPause
        depositTokensForAction
        dnsRegister
        downvote
        execute
        getGovernanceTokenId
        getLockTimeAfterVotingEndsInBlocks
        getMaxActionsPerProposal
        getMinTokenBalanceForProposing
        getProposalActions
        getProposalDescription
        getProposalStatus
        getProposer
        getQuorum
        getTotalDownvotes
        getTotalVotes
        getVotingDelayInBlocks
        getVotingPeriodInBlocks
        initGovernanceModule
        isPaused
        issueToken
        pause
        propose
        queue
        setFeatureFlag
        setLocalRoles
        unpause
        vote
        withdrawGovernanceTokens
   )
}
