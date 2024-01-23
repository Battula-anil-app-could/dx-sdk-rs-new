use dharitri_wasm::{api::VMApi, dharitri_codec::TryStaticCast};

use crate::TxContext;

impl TryStaticCast for TxContext {}

impl VMApi for TxContext {}
