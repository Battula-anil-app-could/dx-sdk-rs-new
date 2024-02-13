use dharitri_wasm::types::{BoxedBytes, MoaxOrDctTokenIdentifier, TokenIdentifier};
use dharitri_wasm_debug::{
    check_managed_top_encode_decode, managed_moax_token_id, managed_token_id,
    managed_token_id_wrapped, DebugApi,
};

#[test]
fn test_moax() {
    let _ = DebugApi::dummy();
    assert!(MoaxOrDctTokenIdentifier::<DebugApi>::moax().is_moax());
}

#[test]
fn test_codec() {
    let api = DebugApi::dummy();
    check_managed_top_encode_decode(
        api.clone(),
        MoaxOrDctTokenIdentifier::<DebugApi>::moax(),
        MoaxOrDctTokenIdentifier::<DebugApi>::MOAX_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &MoaxOrDctTokenIdentifier::<DebugApi>::MOAX_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        api.clone(),
        vec![MoaxOrDctTokenIdentifier::<DebugApi>::moax()],
        expected.as_slice(),
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_dct_identifier() {
    let _ = DebugApi::dummy();

    // valid identifier
    assert!(TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALC-6258d2"[..]).is_valid_dct_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALC123-6258d2"[..]).is_valid_dct_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::<DebugApi>::from_dct_bytes(&b"12345-6258d2"[..]).is_valid_dct_identifier());

    // missing dash
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALC6258d2"[..]).is_valid_dct_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"AL-C6258d2"[..]).is_valid_dct_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"alc-6258d2"[..]).is_valid_dct_identifier());

    // uppercase random chars
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALC-6258D2"[..]).is_valid_dct_identifier());

    // too many random chars
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALC-6258d2ff"[..]).is_valid_dct_identifier());

    // ticker too short
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"AL-6258d2"[..]).is_valid_dct_identifier());

    // ticker too long
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALCCCCCCCCC-6258d2"[..]).is_valid_dct_identifier());
}

#[test]
fn test_managed_token_id_macro() {
    let _ = DebugApi::dummy();
    assert_eq!(
        managed_moax_token_id!(b"MOAX"),
        MoaxOrDctTokenIdentifier::<DebugApi>::moax()
    );
    assert_eq!(
        managed_token_id!(b"ALC-6258d2"),
        TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALC-6258d2"[..])
    );
    assert_eq!(
        managed_token_id_wrapped!(b"ALC-6258d2").unwrap_dct(),
        TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALC-6258d2"[..])
    )
}
