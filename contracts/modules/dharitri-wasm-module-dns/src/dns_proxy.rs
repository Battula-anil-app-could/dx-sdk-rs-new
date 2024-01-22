dharitri_wasm::imports!();

#[dharitri_wasm_derive::proxy]
pub trait Dns {
	#[payable("MOAX")]
	#[endpoint]
	fn register(&self, name: BoxedBytes, #[payment] payment: Self::BigUint);
}
