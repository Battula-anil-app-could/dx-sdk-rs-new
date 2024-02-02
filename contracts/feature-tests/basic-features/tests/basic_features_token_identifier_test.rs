use dharitri_wasm::types::{MoaxOrDctTokenIdentifier, ManagedBuffer, TokenIdentifier};
use dharitri_wasm_debug::*;

use basic_features::token_identifier_features::TokenIdentifierFeatures;

#[test]
fn test_token_identifier_moax() {
    let _ = DebugApi::dummy();
    let bf = basic_features::contract_obj::<DebugApi>();
    let result = bf.token_identifier_moax();
    assert_eq!(MoaxOrDctTokenIdentifier::moax(), result);
}

/// This just tests the contract syntax.
/// For a complete suite of test cases, see `dharitri-wasm-debug/tests/managed_token_identifier_test.rs`.
#[test]
fn test_token_identifier_is_valid() {
    let _ = DebugApi::dummy();
    let bf = basic_features::contract_obj::<DebugApi>();
    let result = bf.token_identifier_is_valid_1(MoaxOrDctTokenIdentifier::dct(
        TokenIdentifier::from(&b"ALC-6258d2"[..]),
    ));
    assert!(result);
    let result = bf.token_identifier_is_valid_1(MoaxOrDctTokenIdentifier::dct(
        TokenIdentifier::from(&b"AL-C6258d2"[..]),
    ));
    assert!(!result);
    let result = bf.token_identifier_is_valid_2(ManagedBuffer::from(&b"12345-6258d2"[..]));
    assert!(result);
    let result = bf.token_identifier_is_valid_2(ManagedBuffer::from(&b"ALCCCCCCCCC-6258d2"[..]));
    assert!(!result);
}

#[test]
fn test_token_identifier_compare() {
    let _ = DebugApi::dummy();

    let token_id = TokenIdentifier::<DebugApi>::from(&b"ALC-6258d2"[..]);
    let dct_token_id = MoaxOrDctTokenIdentifier::dct(token_id.clone());
    let wrong_dct_token_id =
        MoaxOrDctTokenIdentifier::dct(TokenIdentifier::from(&b"AAA-111111"[..]));
    let moax_token_id = MoaxOrDctTokenIdentifier::moax();

    assert!(token_id == dct_token_id);
    assert!(dct_token_id == token_id);

    assert!(token_id != moax_token_id);
    assert!(moax_token_id != token_id);

    assert!(token_id != wrong_dct_token_id);
    assert!(wrong_dct_token_id != token_id);
}
