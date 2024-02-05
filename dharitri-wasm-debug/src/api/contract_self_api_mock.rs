use super::big_int_api_mock::*;
use super::big_uint_api_mock::*;
use crate::TxContext;

impl dharitri_wasm::api::ContractBase for TxContext {
	type BigUint = RustBigUint;
	type BigInt = RustBigInt;
	type Storage = Self;
	type CallValue = Self;
	type SendApi = Self;
	type BlockchainApi = Self;
	type CryptoApi = Self;
	type LogApi = Self;
	type ErrorApi = Self;

	fn get_storage_raw(&self) -> Self::Storage {
		self.clone()
	}

	fn call_value(&self) -> Self::CallValue {
		self.clone()
	}

	fn send(&self) -> Self::SendApi {
		self.clone()
	}

	fn blockchain(&self) -> Self::BlockchainApi {
		self.clone()
	}

	fn crypto(&self) -> Self::CryptoApi {
		self.clone()
	}

	fn log_api_raw(&self) -> Self::LogApi {
		self.clone()
	}

	fn error_api(&self) -> Self::ErrorApi {
		self.clone()
	}
}
