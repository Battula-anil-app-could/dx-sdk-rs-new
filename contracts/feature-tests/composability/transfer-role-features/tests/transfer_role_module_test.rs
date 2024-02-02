use dharitri_wasm::types::{DctLocalRole, DctTokenPayment, ManagedArgBuffer, ManagedVec};
use dharitri_wasm_debug::{
    managed_address, managed_biguint, managed_buffer, managed_token_id, rust_biguint,
    testing_framework::BlockchainStateWrapper,
};
use dharitri_wasm_modules::transfer_role_proxy::TransferRoleProxyModule;

static TRANSFER_TOKEN_ID: &[u8] = b"TRANSFER-123456";
static ACCEPT_FUNDS_FUNC_NAME: &[u8] = b"accept_funds";
static REJECT_FUNDS_FUNC_NAME: &[u8] = b"reject_funds";

#[test]
fn test_transfer_role_module() {
    let rust_zero = rust_biguint!(0);
    let mut b_mock = BlockchainStateWrapper::new();
    let user = b_mock.create_user_account(&rust_zero);
    b_mock.set_dct_balance(&user, TRANSFER_TOKEN_ID, &rust_biguint!(1_000));

    let owner = b_mock.create_user_account(&rust_zero);
    let sc_with_transfer_role = b_mock.create_sc_account(
        &rust_zero,
        Some(&owner),
        transfer_role_features::contract_obj,
        "wasm 1",
    );
    let sc_dest = b_mock.create_sc_account(&rust_zero, Some(&owner), vault::contract_obj, "wasm 2");

    // Note: The role restrictions are not currently implemented in the mock
    b_mock.set_dct_local_roles(
        sc_with_transfer_role.address_ref(),
        TRANSFER_TOKEN_ID,
        &[DctLocalRole::Transfer],
    );

    // transfer to user
    b_mock
        .execute_dct_transfer(
            &user,
            &sc_with_transfer_role,
            TRANSFER_TOKEN_ID,
            0,
            &rust_biguint!(100),
            |sc| {
                let payments = ManagedVec::from_single_item(DctTokenPayment::new(
                    managed_token_id!(TRANSFER_TOKEN_ID),
                    0,
                    managed_biguint!(100),
                ));
                sc.transfer_to_user(
                    managed_address!(&user),
                    managed_address!(&owner),
                    payments,
                    managed_buffer!(b"enjoy"),
                );
            },
        )
        .assert_ok();

    b_mock.check_dct_balance(&user, TRANSFER_TOKEN_ID, &rust_biguint!(900));
    b_mock.check_dct_balance(&owner, TRANSFER_TOKEN_ID, &rust_biguint!(100));

    // transfer to sc - ok
    b_mock
        .execute_dct_transfer(
            &user,
            &sc_with_transfer_role,
            TRANSFER_TOKEN_ID,
            0,
            &rust_biguint!(100),
            |sc| {
                let payments = ManagedVec::from_single_item(DctTokenPayment::new(
                    managed_token_id!(TRANSFER_TOKEN_ID),
                    0,
                    managed_biguint!(100),
                ));
                sc.transfer_to_contract_raw(
                    managed_address!(&user),
                    managed_address!(sc_dest.address_ref()),
                    payments,
                    managed_buffer!(ACCEPT_FUNDS_FUNC_NAME),
                    ManagedArgBuffer::new(),
                    None,
                );
            },
        )
        .assert_ok();

    b_mock.check_dct_balance(&user, TRANSFER_TOKEN_ID, &rust_biguint!(800));
    b_mock.check_dct_balance(
        sc_dest.address_ref(),
        TRANSFER_TOKEN_ID,
        &rust_biguint!(100),
    );

    // transfer to sc - err, wrong number of args
    b_mock
        .execute_dct_transfer(
            &user,
            &sc_with_transfer_role,
            TRANSFER_TOKEN_ID,
            0,
            &rust_biguint!(100),
            |sc| {
                let payments = ManagedVec::from_single_item(DctTokenPayment::new(
                    managed_token_id!(TRANSFER_TOKEN_ID),
                    0,
                    managed_biguint!(100),
                ));

                sc.transfer_to_contract_raw(
                    managed_address!(&user),
                    managed_address!(sc_dest.address_ref()),
                    payments,
                    managed_buffer!(REJECT_FUNDS_FUNC_NAME),
                    ManagedArgBuffer::new(),
                    None,
                );
            },
        )
        .assert_ok();

    b_mock.check_dct_balance(&user, TRANSFER_TOKEN_ID, &rust_biguint!(800));
    b_mock.check_dct_balance(
        sc_dest.address_ref(),
        TRANSFER_TOKEN_ID,
        &rust_biguint!(100),
    );
}
