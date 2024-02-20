#![no_std]
#![allow(unused_attributes)]

dharitri_sc::imports!();
dharitri_sc::derive_imports!();

mod deposit_info;

use deposit_info::{DepositInfo, Fee};

pub const SECONDS_PER_ROUND: u64 = 6;
pub use dharitri_sc::api::{ED25519_KEY_BYTE_LEN, ED25519_SIGNATURE_BYTE_LEN};

#[dharitri_sc::contract]
pub trait DigitalCash {
    #[init]
    fn init(&self, fee: BigUint) {
        self.fee().set(fee);
    }

    //endpoints

    #[endpoint]
    #[payable("*")]
    fn fund(&self, address: ManagedAddress, valability: u64) {
        require!(!self.deposit(&address).is_empty(), "fees not covered");

        let dct_payment = self.call_value().all_dct_transfers().clone_value();
        let moax_payment = self.call_value().moax_value().clone_value();

        let num_tokens = (moax_payment != BigUint::zero()) as usize + dct_payment.len();

        require!(num_tokens > 0, "amount must be greater than 0");

        let fee = self.fee().get();

        self.deposit(&address).update(|deposit| {
            require!(
                deposit.moax_funds == BigUint::zero() && deposit.dct_funds.is_empty(),
                "key already used"
            );
            require!(
                fee * num_tokens as u64 <= deposit.fees.value,
                "cannot deposit funds without covering the fee cost first"
            );

            deposit.fees.num_token_to_transfer += num_tokens;
            deposit.valability = valability;
            deposit.expiration_round = self.get_expiration_round(valability);
            deposit.dct_funds = dct_payment;
            deposit.moax_funds = moax_payment;
        });
    }

    #[endpoint]
    fn withdraw(&self, address: ManagedAddress) {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let block_round = self.blockchain().get_block_round();

        let deposit = self.deposit(&address).get();

        require!(
            deposit.expiration_round < block_round,
            "withdrawal has not been available yet"
        );

        let moax_funds = deposit.moax_funds + deposit.fees.value;
        if moax_funds != BigUint::zero() {
            self.send()
                .direct_moax(&deposit.depositor_address, &moax_funds);
        }

        if !deposit.dct_funds.is_empty() {
            self.send()
                .direct_multi(&deposit.depositor_address, &deposit.dct_funds);
        }

        self.deposit(&address).clear();
    }

    #[endpoint]
    fn claim(
        &self,
        address: ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let caller_address = self.blockchain().get_caller();
        self.require_signature(&address, &caller_address, signature);

        let block_round = self.blockchain().get_block_round();

        let fee = self.fee().get();

        self.deposit(&address).update(|deposit| {
            require!(deposit.expiration_round >= block_round, "deposit expired");
            let num_tokens_transfered = &deposit.get_num_tokens();
            let fee_cost = fee * *num_tokens_transfered as u64;

            deposit.fees.num_token_to_transfer -= num_tokens_transfered;
            deposit.fees.value -= &fee_cost;

            self.collected_fees()
                .update(|collected_fees| *collected_fees += fee_cost);

            if deposit.moax_funds != BigUint::zero() {
                self.send()
                    .direct_moax(&caller_address, &deposit.moax_funds);
            }

            if !deposit.dct_funds.is_empty() {
                self.send()
                    .direct_multi(&caller_address, &deposit.dct_funds);
            }

            if deposit.fees.value > 0 {
                self.send()
                    .direct_moax(&deposit.depositor_address, &deposit.fees.value);
            }
        });

        self.deposit(&address).clear();
    }

    #[endpoint]
    #[only_owner]
    fn claim_fees(&self) {
        let caller_address = self.blockchain().get_caller();
        let fees = self.collected_fees().get();

        self.send().direct_moax(&caller_address, &fees);
        self.collected_fees().clear();
    }

    fn require_signature(
        &self,
        address: &ManagedAddress,
        caller_address: &ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let addr = address.as_managed_buffer();
        let message = caller_address.as_managed_buffer();
        require!(
            self.crypto()
                .verify_ed25519(addr, message, signature.as_managed_buffer()),
            "invalid signature"
        );
    }

    #[endpoint]
    #[payable("MOAX")]
    fn deposit_fees(&self, address: ManagedAddress) {
        let payment = self.call_value().moax_value().clone_value();
        let caller_address = self.blockchain().get_caller();

        if self.deposit(&address).is_empty() {
            let new_deposit = DepositInfo {
                depositor_address: caller_address,
                dct_funds: ManagedVec::new(),
                moax_funds: BigUint::zero(),
                valability: 0,
                expiration_round: 0,
                fees: Fee {
                    num_token_to_transfer: 0,
                    value: payment,
                },
            };
            self.deposit(&address).set(new_deposit)
        } else {
            self.deposit(&address)
                .update(|deposit| deposit.fees.value += payment);
        }
    }

    #[endpoint]
    fn forward(
        &self,
        address: ManagedAddress,
        forward_address: ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        require!(
            !self.deposit(&forward_address).is_empty(),
            "cannot deposit funds without covering the fee cost first"
        );

        let caller_address = self.blockchain().get_caller();
        let fee = self.fee().get();
        self.require_signature(&address, &caller_address, signature);

        let mut forwarded_deposit = self.deposit(&address).get();
        let num_tokens = forwarded_deposit.get_num_tokens();
        self.deposit(&forward_address).update(|deposit| {
            require!(
                deposit.moax_funds == BigUint::zero() && deposit.dct_funds.is_empty(),
                "key already used"
            );
            require!(
                &fee * num_tokens as u64 <= deposit.fees.value,
                "cannot forward funds without the owner covering the fee cost first"
            );

            deposit.fees.num_token_to_transfer += num_tokens;
            deposit.valability = forwarded_deposit.valability;
            deposit.expiration_round = self.get_expiration_round(forwarded_deposit.valability);
            deposit.dct_funds = forwarded_deposit.dct_funds;
            deposit.moax_funds = forwarded_deposit.moax_funds;
        });

        let forward_fee = &fee * num_tokens as u64;

        forwarded_deposit.fees.value -= &forward_fee;

        self.collected_fees()
            .update(|collected_fees| *collected_fees += forward_fee);

        if forwarded_deposit.fees.value > 0 {
            self.send().direct_moax(
                &forwarded_deposit.depositor_address,
                &forwarded_deposit.fees.value,
            );
        }

        self.deposit(&address).clear();
    }

    //views

    #[view(amount)]
    fn get_amount(
        &self,
        address: ManagedAddress,
        token: MoaxOrDctTokenIdentifier,
        nonce: u64,
    ) -> BigUint {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let mut amount = BigUint::zero();

        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let deposit = self.deposit(&address).get();
        if token.is_moax() {
            amount = deposit.moax_funds;
        } else {
            for dct in deposit.dct_funds.into_iter() {
                if dct.token_identifier == token && dct.token_nonce == nonce {
                    amount = dct.amount;
                }
            }
        }

        amount
    }

    //private functions

    fn get_expiration_round(&self, valability: u64) -> u64 {
        let valability_rounds = valability / SECONDS_PER_ROUND;
        self.blockchain().get_block_round() + valability_rounds
    }

    //storage

    #[view]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<DepositInfo<Self::Api>>;

    #[storage_mapper("fee")]
    fn fee(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("collected_fees")]
    fn collected_fees(&self) -> SingleValueMapper<BigUint>;
}
