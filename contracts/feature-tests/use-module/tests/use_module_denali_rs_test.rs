mod user_builtin {
	dharitri_wasm::imports!();

	#[dharitri_wasm_derive::proxy]
	pub trait UserBuiltin {
		#[endpoint(SetUserName)]
		fn set_user_name(&self, name: &BoxedBytes) -> Self::BigUint;
	}
}

mod dns_mock {
	dharitri_wasm::imports!();

	#[dharitri_wasm_derive::contract]
	pub trait DnsMock {
		#[proxy]
		fn user_builtin_proxy(&self, to: Address) -> super::user_builtin::Proxy<Self::SendApi>;

		#[payable("MOAX")]
		#[endpoint]
		fn register(
			&self,
			name: BoxedBytes,
			#[payment] _payment: Self::BigUint,
		) -> AsyncCall<Self::SendApi> {
			let address = self.blockchain().get_caller();
			self.user_builtin_proxy(address)
				.set_user_name(&name)
				.async_call()
		}
	}
}

use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/use-module.wasm",
		Box::new(|context| Box::new(use_module::contract_obj(context))),
	);

	contract_map.register_contract(
		"file:../test-wasm/dns.wasm",
		Box::new(|context| Box::new(dns_mock::contract_obj(context))),
	);

	contract_map
}

#[test]
fn use_module_dns_register_rs() {
	dharitri_wasm_debug::denali_rs("denali/use_module_dns_register.scen.json", &contract_map());
}

#[test]
fn use_module_features_rs() {
	dharitri_wasm_debug::denali_rs("denali/use_module_features.scen.json", &contract_map());
}

#[test]
fn use_module_internal_rs() {
	dharitri_wasm_debug::denali_rs("denali/use_module_internal.scen.json", &contract_map());
}

#[test]
fn use_module_pause_rs() {
	dharitri_wasm_debug::denali_rs("denali/use_module_pause.scen.json", &contract_map());
}
