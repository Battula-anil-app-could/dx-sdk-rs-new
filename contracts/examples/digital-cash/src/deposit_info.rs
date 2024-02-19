use dharitri_sc::{
    api::ManagedTypeApi,
    types::{MoaxOrDctTokenIdentifier, MoaxOrDctTokenPayment, ManagedAddress},
};

dharitri_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub depositor_address: ManagedAddress<M>,
    pub payment: MoaxOrDctTokenPayment<M>,
    pub expiration_round: u64,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone, ManagedVecItem)]
pub struct FundType<M: ManagedTypeApi> {
    pub token: MoaxOrDctTokenIdentifier<M>,
    pub nonce: u64,
}
