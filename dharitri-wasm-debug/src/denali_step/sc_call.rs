use crate::execute_helper_functions::{self, *};
use denali::{TxCall, TxExpect};
use num_bigint::BigUint;
use num_traits::Zero;

use crate::{BlockchainMock, ContractMap, TxContext, TxInput};
pub fn execute(
    state: &mut BlockchainMock,
    contract_map: &ContractMap<TxContext>,
    tx_id: &str,
    tx: &TxCall,
    expect: &Option<TxExpect>,
) {
    let mut dct_value = BigUint::zero();
    let mut dct_token_identifier = Vec::new();
    if let Some(value) = tx.dct_value.as_ref() {
        dct_value = value.dct_value.value.clone();
        dct_token_identifier = value.dct_token_identifier.value.clone();
    };
    let tx_input = TxInput {
        from: tx.from.value.into(),
        to: tx.to.value.into(),
        call_value: tx.call_value.value.clone(),
        dct_value,
        dct_token_identifier,
        func_name: tx.function.as_bytes().to_vec(),
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: tx.gas_limit.value,
        gas_price: tx.gas_price.value,
        tx_hash: execute_helper_functions::generate_tx_hash_dummy(tx_id),
    };
    state.increase_nonce(&tx_input.from);
    let tx_result = sc_call_with_async_and_callback(tx_input, state, contract_map).unwrap();
    if let Some(tx_expect) = expect {
        check_tx_output(tx_id, tx_expect, &tx_result);
    }
}
