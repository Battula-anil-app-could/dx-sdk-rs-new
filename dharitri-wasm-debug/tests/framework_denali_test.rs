use dharitri_wasm_debug::*;

// These tests don't really test any contract, but the testing framework itslef.

fn contract_map() -> ContractMap<TxContext> {
    ContractMap::new()
}

/// Checks that externalSteps work fine.
#[test]
fn external_steps_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/external_steps/external_steps.scen.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_addr_len_err1_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-addr-len.err1.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_addr_len_err2_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-addr-len.err2.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_sc_addr_err1_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-sc-addr.err1.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_sc_addr_err2_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-sc-addr.err2.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_sc_addr_err3_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-sc-addr.err3.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_balance_err_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-balance.err.json",
        &contract_map(),
    );
}

#[test]
fn set_check_balance_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-balance.scen.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_code_err_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-code.err.json",
        &contract_map(),
    );
}

#[test]
fn set_check_code() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-code.scen.json",
        &contract_map(),
    );
}

#[test]
#[ignore]
#[should_panic]
fn set_check_dct_err_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-dct.err.json",
        &contract_map(),
    );
}

#[test]
#[ignore]
fn set_check_dct_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-dct.scen.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_nonce_err_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-nonce.err.json",
        &contract_map(),
    );
}

#[test]
fn set_check_nonce_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-nonce.scen.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err1_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err1.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err2_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err2.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err3_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err3.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err4_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err4.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err5_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err5.json",
        &contract_map(),
    );
}

#[test]
fn set_check_storage_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.scen.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_username_err_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-username.err.json",
        &contract_map(),
    );
}

#[test]
fn set_check_username_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-username.scen.json",
        &contract_map(),
    );
}

#[test]
#[ignore]
fn builtin_func_dct_transfer() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/builtin-func-dct-transfer.scen.json",
        &contract_map(),
    );
}

#[test]
#[ignore]
fn dct_non_zero_balance_check_err_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/dct-non-zero-balance-check-err.scen.json",
        &contract_map(),
    );
}

#[test]
#[ignore]
fn dct_zero_balance_check_err_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/dct-zero-balance-check-err.scen.json",
        &contract_map(),
    );
}

#[test]
#[ignore]
fn multi_transfer_dct_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/multi-transfer-dct.scen.json",
        &contract_map(),
    );
}

#[test]
fn transfer_moax_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/transfer-moax.scen.json", &contract_map());
}

#[test]
#[ignore]
fn transfer_dct_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/transfer-dct.scen.json", &contract_map());
}

#[test]
fn validator_reward_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/validatorReward.scen.json", &contract_map());
}
