use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

use super::{
    change_owner_mock::execute_change_owner, dct_local_burn::execute_local_burn,
    dct_local_mint::execute_local_mint, dct_multi_transfer_mock::execute_dct_multi_transfer,
    dct_nft_add_quantity_mock::execute_nft_add_quantity, dct_nft_burn_mock::execute_nft_burn,
    dct_nft_create_mock::execute_dct_nft_create,
    dct_nft_transfer_mock::execute_dct_nft_transfer, dct_transfer_mock::execute_dct_transfer,
    set_username_mock::execute_set_username, upgrade_contract::execute_upgrade_contract,
};

use dharitri_wasm::api::{
    CHANGE_OWNER_BUILTIN_FUNC_NAME, DCT_LOCAL_BURN_FUNC_NAME, DCT_LOCAL_MINT_FUNC_NAME,
    DCT_MULTI_TRANSFER_FUNC_NAME, DCT_NFT_ADD_QUANTITY_FUNC_NAME, DCT_NFT_BURN_FUNC_NAME,
    DCT_NFT_CREATE_FUNC_NAME, DCT_NFT_TRANSFER_FUNC_NAME, DCT_TRANSFER_FUNC_NAME,
    SET_USERNAME_FUNC_NAME, UPGRADE_CONTRACT_FUNC_NAME,
};

pub fn execute_builtin_function_or_default(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    match tx_input.func_name.as_slice() {
        DCT_LOCAL_MINT_FUNC_NAME => execute_local_mint(tx_input, tx_cache),
        DCT_LOCAL_BURN_FUNC_NAME => execute_local_burn(tx_input, tx_cache),
        DCT_MULTI_TRANSFER_FUNC_NAME => execute_dct_multi_transfer(tx_input, tx_cache),
        DCT_NFT_TRANSFER_FUNC_NAME => execute_dct_nft_transfer(tx_input, tx_cache),
        DCT_NFT_CREATE_FUNC_NAME => execute_dct_nft_create(tx_input, tx_cache),
        DCT_NFT_ADD_QUANTITY_FUNC_NAME => execute_nft_add_quantity(tx_input, tx_cache),
        DCT_NFT_BURN_FUNC_NAME => execute_nft_burn(tx_input, tx_cache),
        DCT_TRANSFER_FUNC_NAME => execute_dct_transfer(tx_input, tx_cache),
        CHANGE_OWNER_BUILTIN_FUNC_NAME => execute_change_owner(tx_input, tx_cache),
        SET_USERNAME_FUNC_NAME => execute_set_username(tx_input, tx_cache),
        UPGRADE_CONTRACT_FUNC_NAME => execute_upgrade_contract(tx_input, tx_cache),
        _ => default_execution(tx_input, tx_cache),
    }
}
