use num_bigint::BigUint;

use crate::{tx_execution::BuiltinFunctionContainer, types::VMAddress};

use super::{async_call_tx_input, CallType, TxResult, TxTokenTransfer};

#[derive(Default)]
pub struct BackTransfers {
    pub call_value: BigUint,
    pub dct_transfers: Vec<TxTokenTransfer>,
}

impl BackTransfers {
    pub fn empty() -> Self {
        BackTransfers::default()
    }

    pub fn new_from_result(
        own_address: &VMAddress,
        result: &TxResult,
        builtin_functions: &BuiltinFunctionContainer,
    ) -> Self {
        let mut bt = BackTransfers::default();

        if result.result_status != 0 {
            return bt;
        }

        for call in &result.all_calls {
            // TODO: refactor, check type

            if call.endpoint_name.is_empty() {
                bt.call_value += &call.call_value;
                continue;
            }

            let tx_input = async_call_tx_input(call, CallType::BackTransfer);
            let mut token_transfers = builtin_functions.extract_token_transfers(&tx_input);
            if &token_transfers.real_recipient == own_address {
                bt.dct_transfers.append(&mut token_transfers.transfers);
            }
        }

        bt
    }

    pub fn merge(&mut self, other: &BackTransfers) {
        self.call_value += &other.call_value;
        self.dct_transfers.extend_from_slice(&other.dct_transfers);
    }
}

// func (host *vmHost) addNewBackTransfersFromVMOutput(vmOutput *vmcommon.VMOutput, parent, child []byte) {
// 	if vmOutput == nil || vmOutput.ReturnCode != vmcommon.Ok {
// 		return
// 	}
// 	callerOutAcc, ok := vmOutput.OutputAccounts[string(parent)]
// 	if !ok {
// 		return
// 	}

// 	for _, transfer := range callerOutAcc.OutputTransfers {
// 		if !bytes.Equal(transfer.SenderAddress, child) {
// 			continue
// 		}
// 		if transfer.CallType == vm.AsynchronousCallBack {
// 			continue
// 		}

// 		if transfer.Value.Cmp(vmhost.Zero) > 0 {
// 			if len(transfer.Data) == 0 {
// 				host.managedTypesContext.AddValueOnlyBackTransfer(transfer.Value)
// 			}
// 			continue
// 		}

// 		dctTransfers, isWithoutExec := host.isDCTTransferWithoutExecution(transfer.Data, parent, child)
// 		if !isWithoutExec {
// 			continue
// 		}

// 		host.managedTypesContext.AddBackTransfers(dctTransfers.DCTTransfers)
// 	}
// }
