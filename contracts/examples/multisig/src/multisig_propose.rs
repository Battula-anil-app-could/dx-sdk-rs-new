use crate::action::Action;

dharitri_wasm::imports!();

/// Contains all events that can be emitted by the contract.
#[dharitri_wasm::module]
pub trait MultisigProposeModule: crate::multisig_state::MultisigStateModule {
    fn propose_action(&self, action: Action<Self::Api>) -> SCResult<usize> {
        let caller_address = self.blockchain().get_caller();
        let caller_id = self.user_mapper().get_user_id(&caller_address);
        let caller_role = self.get_user_id_to_role(caller_id);
        require!(
            caller_role.can_propose(),
            "only board members and proposers can propose"
        );

        let action_id = self.action_mapper().push(&action);
        if caller_role.can_sign() {
            // also sign
            // since the action is newly created, the caller can be the only signer
            self.action_signer_ids(action_id).insert(caller_id);
        }

        Ok(action_id)
    }

    /// Initiates board member addition process.
    /// Can also be used to promote a proposer to board member.
    #[endpoint(proposeAddBoardMember)]
    fn propose_add_board_member(&self, board_member_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::AddBoardMember(board_member_address))
    }

    /// Initiates proposer addition process..
    /// Can also be used to demote a board member to proposer.
    #[endpoint(proposeAddProposer)]
    fn propose_add_proposer(&self, proposer_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::AddProposer(proposer_address))
    }

    /// Removes user regardless of whether it is a board member or proposer.
    #[endpoint(proposeRemoveUser)]
    fn propose_remove_user(&self, user_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::RemoveUser(user_address))
    }

    #[endpoint(proposeChangeQuorum)]
    fn propose_change_quorum(&self, new_quorum: usize) -> SCResult<usize> {
        self.propose_action(Action::ChangeQuorum(new_quorum))
    }

    /// Propose a transaction in which the contract can send MOAX
    /// and optionally execute a contract endpoint or builtin function.
    #[endpoint(proposeSendMoax)]
    fn propose_send_moax(
        &self,
        to: ManagedAddress,
        amount: BigUint,
        #[var_args] opt_function: OptionalArg<ManagedBuffer>,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        let endpoint_name = match opt_function {
            OptionalArg::Some(data) => data,
            OptionalArg::None => ManagedBuffer::new(),
        };
        self.propose_action(Action::SendMOAX {
            to,
            amount,
            endpoint_name,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSendDct)]
    fn propose_send_dct(
        &self,
        to: ManagedAddress,
        dct_payment_args: ManagedCountedVarArgs<DctTokenPaymentMultiArg<Self::Api>>,
        #[var_args] opt_function: OptionalArg<ManagedBuffer>,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        let mut dct_payments_vec = ManagedVec::new();
        for payment_args in dct_payment_args.into_vec().into_iter() {
            dct_payments_vec.push(payment_args.into_dct_token_payment());
        }
        let endpoint_name = match opt_function {
            OptionalArg::Some(data) => data,
            OptionalArg::None => ManagedBuffer::new(),
        };
        self.propose_action(Action::SendDCT {
            to,
            dct_payments: dct_payments_vec,
            endpoint_name,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSCDeploy)]
    fn propose_sc_deploy(
        &self,
        amount: BigUint,
        code: ManagedBuffer,
        code_metadata: CodeMetadata,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        self.propose_action(Action::SCDeploy {
            amount,
            code,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSCDeployFromSource)]
    fn propose_sc_deploy_from_source(
        &self,
        amount: BigUint,
        source: ManagedAddress,
        code_metadata: CodeMetadata,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        self.propose_action(Action::SCDeployFromSource {
            amount,
            source,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSCUpgrade)]
    fn propose_sc_upgrade(
        &self,
        sc_address: ManagedAddress,
        amount: BigUint,
        code: ManagedBuffer,
        code_metadata: CodeMetadata,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        self.propose_action(Action::SCUpgrade {
            sc_address,
            amount,
            code,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSCUpgradeFromSource)]
    fn propose_sc_upgrade_from_source(
        &self,
        sc_address: ManagedAddress,
        amount: BigUint,
        source: ManagedAddress,
        code_metadata: CodeMetadata,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        self.propose_action(Action::SCUpgradeFromSource {
            sc_address,
            amount,
            source,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }
}
