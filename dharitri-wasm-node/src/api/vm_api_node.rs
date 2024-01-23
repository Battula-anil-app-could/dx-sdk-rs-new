use crate::ArwenApiImpl;
use dharitri_wasm::{api::VMApi, dharitri_codec::TryStaticCast};

impl TryStaticCast for ArwenApiImpl {}

impl VMApi for ArwenApiImpl {}
