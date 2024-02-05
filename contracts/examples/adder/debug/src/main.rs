use adder::*;
use dharitri_wasm_debug::*;

fn main() {
	let mut contract_map = ContractMap::<TxContext>::new();
	contract_map.register_contract(
		"file:../output/adder.wasm",
		Box::new(|context| Box::new(AdderImpl::new(context))),
	);

	parse_execute_denali("examples/adder/denali/adder.scen.json", &contract_map);

	println!("Ok");
}
