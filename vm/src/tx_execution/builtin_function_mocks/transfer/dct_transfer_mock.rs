use crate::{
    tx_execution::{
        builtin_function_names::DCT_TRANSFER_FUNC_NAME, BlockchainVMRef,
        BuiltinFunctionDctTransferInfo,
    },
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

use super::{
    super::builtin_func_trait::BuiltinFunction,
    transfer_common::{
        execute_transfer_builtin_func, extract_transfer_info, ParsedTransferBuiltinFunCall,
        RawDctTransfer,
    },
};

pub struct DCTTransfer;

impl BuiltinFunction for DCTTransfer {
    fn name(&self) -> &str {
        DCT_TRANSFER_FUNC_NAME
    }

    fn extract_dct_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionDctTransferInfo {
        if let Ok(parsed_tx) = try_parse_input(tx_input) {
            extract_transfer_info(parsed_tx)
        } else {
            BuiltinFunctionDctTransferInfo::empty(tx_input)
        }
    }

    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        vm: &BlockchainVMRef,
        f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        match try_parse_input(&tx_input) {
            Ok(parsed_tx) => {
                execute_transfer_builtin_func(vm, parsed_tx, self.name(), tx_input, tx_cache, f)
            },
            Err(message) => {
                let err_result = TxResult::from_vm_error(message);
                (err_result, BlockchainUpdate::empty())
            },
        }
    }
}

fn try_parse_input(tx_input: &TxInput) -> Result<ParsedTransferBuiltinFunCall, &'static str> {
    if tx_input.args.len() < 2 {
        return Err("DCTTransfer too few arguments");
    }

    let token_identifier = tx_input.args[0].clone();
    let value_bytes = tx_input.args[1].clone();

    let func_name = tx_input.func_name_from_arg_index(2);
    let args = if tx_input.args.len() > 2 {
        tx_input.args[3..].to_vec()
    } else {
        Vec::new()
    };

    Ok(ParsedTransferBuiltinFunCall {
        destination: tx_input.to.clone(),
        raw_dct_transfers: vec![RawDctTransfer {
            token_identifier,
            nonce_bytes: Vec::new(),
            value_bytes,
        }],
        func_name,
        args,
    })
}
