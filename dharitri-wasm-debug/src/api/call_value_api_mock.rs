use crate::{TxContext, TxPanic};
use dharitri_wasm::{
    api::CallValueApi,
    err_msg,
    types::{BigUint, DctTokenType, ManagedBuffer, TokenIdentifier},
};

impl CallValueApi for TxContext {
    fn check_not_payable(&self) {
        if self.moax_value() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_MOAX.to_vec(),
            });
        }
        if self.dct_value() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_DCT.to_vec(),
            });
        }
    }

    #[inline]
    fn moax_value(&self) -> BigUint<Self> {
        self.insert_new_big_uint(self.tx_input_box.call_value.clone())
    }

    #[inline]
    fn dct_value(&self) -> BigUint<Self> {
        self.insert_new_big_uint(self.tx_input_box.dct_value.clone())
    }

    #[inline]
    fn token(&self) -> TokenIdentifier<Self> {
        ManagedBuffer::new_from_bytes(
            self.clone(),
            self.tx_input_box.dct_token_identifier.as_slice(),
        )
        .into()
    }

    #[inline]
    fn dct_token_nonce(&self) -> u64 {
        // TODO: Add DCT nonce in mock
        0u64
    }

    #[inline]
    fn dct_token_type(&self) -> DctTokenType {
        // TODO: Add DCT token type in mock
        DctTokenType::Fungible
    }

    // TODO: Mock multi-transfers

    #[inline]
    fn dct_num_transfers(&self) -> usize {
        0
    }

    #[inline]
    fn dct_value_by_index(&self, _index: usize) -> BigUint<Self> {
        self.insert_new_big_uint_zero()
    }

    #[inline]
    fn token_by_index(&self, _index: usize) -> TokenIdentifier<Self> {
        TokenIdentifier::moax(self.clone())
    }

    #[inline]
    fn dct_token_nonce_by_index(&self, _index: usize) -> u64 {
        0
    }

    #[inline]
    fn dct_token_type_by_index(&self, _index: usize) -> DctTokenType {
        DctTokenType::Fungible
    }
}
