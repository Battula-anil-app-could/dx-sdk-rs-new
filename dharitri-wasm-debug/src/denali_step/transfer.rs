use denali::TxTransfer;

use crate::BlockchainMock;

pub fn execute(state: &mut BlockchainMock, tx: &TxTransfer) {
    let sender_address = &tx.from.value.into();
    state.increase_nonce(sender_address);
    state
        .subtract_tx_payment(sender_address, &tx.value.value)
        .unwrap();
    let recipient_address = &tx.to.value.into();
    state.increase_balance(recipient_address, &tx.value.value);
    let dct_token_identifier = tx.dct_token_identifier.value.clone();
    let dct_value = tx.dct_value.value.clone();

    if !dct_token_identifier.is_empty() && dct_value > 0u32.into() {
        state.substract_dct_balance(sender_address, &dct_token_identifier[..], &dct_value);
        state.increase_dct_balance(recipient_address, &dct_token_identifier[..], &dct_value);
    }
}
