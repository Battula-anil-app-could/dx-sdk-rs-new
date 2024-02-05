use adder::*;
use dharitri_wasm_debug::api::RustBigInt;
use dharitri_wasm_debug::TxContext;

#[test]
fn test_add() {
	let adder = adder::contract_obj(TxContext::dummy());

	adder.init(&RustBigInt::from(5));
	assert_eq!(RustBigInt::from(5), adder.get_sum());

	let _ = adder.add(&RustBigInt::from(7));
	assert_eq!(RustBigInt::from(12), adder.get_sum());

	let _ = adder.add(&RustBigInt::from(1));
	assert_eq!(RustBigInt::from(13), adder.get_sum());
}
