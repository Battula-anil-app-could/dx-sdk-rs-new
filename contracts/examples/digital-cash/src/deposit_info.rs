use dharitri_sc::{
    api::ManagedTypeApi,
    types::{BigUint, DctTokenPayment, ManagedAddress, ManagedVec},
};

dharitri_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub depositor_address: ManagedAddress<M>,
    pub dct_funds: ManagedVec<M, DctTokenPayment<M>>,
    pub moax_funds: BigUint<M>,
    pub valability: u64,
    pub expiration_round: u64,
    pub fees: Fee<M>,
}

impl<M> DepositInfo<M>
where
    M: ManagedTypeApi,
{
    pub fn get_num_tokens(&self) -> usize {
        (self.moax_funds != BigUint::zero()) as usize + self.dct_funds.len()
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Default)]
pub struct Fee<M: ManagedTypeApi> {
    pub num_token_to_transfer: usize,
    pub value: BigUint<M>,
}
