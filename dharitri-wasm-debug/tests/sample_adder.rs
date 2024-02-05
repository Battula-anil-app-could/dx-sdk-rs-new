// The purpose of this test is to directly showcase how the various
// API traits are being used, without the aid of macros.
// All this code is of course always macro-generated.
//
// Since it is more difficult to debug macros directly,
// it is helpful to keep this test as a reference for macro development
// and maintenance.

use dharitri_wasm::types::Address;

use crate::module_1::VersionModule;

mod module_1 {
	dharitri_wasm::imports!();

	/////////////////////////////////////////////////////////////////////////////////////////////////
	//////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////////////////////////////////////////
	pub trait VersionModule: dharitri_wasm::api::ContractBase + Sized
	where
		for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
		for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
	{
		fn version(&self) -> Self::BigInt;

		fn some_async(&self) -> AsyncCall<Self::SendApi>;

		fn callback(&self);
	}

	/////////////////////////////////////////////////////////////////////////////////////////////////
	//////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////////////////////////////////////////
	pub trait AutoImpl: dharitri_wasm::api::ContractBase {}

	impl<C> VersionModule for C
	where
		C: AutoImpl,
		for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
		for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
	{
		fn version(&self) -> Self::BigInt {
			Self::BigInt::from(100)
		}

		fn some_async(&self) -> AsyncCall<Self::SendApi> {
			panic!("wooo")
		}

		fn callback(&self) {}
	}

	pub trait EndpointWrappers: VersionModule + dharitri_wasm::api::ContractPrivateApi
	where
		for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
		for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
	{
		#[inline]
		fn call_version(&self) {
			self.call_value().check_not_payable();
			let result = self.version();
			dharitri_wasm::io::EndpointResult::finish(&result, self.finish_api())
		}

		fn call_some_async(&self) {
			let result = self.some_async();
			dharitri_wasm::io::EndpointResult::finish(&result, self.finish_api())
		}

		fn call(&self, fn_name: &[u8]) -> bool {
			if match fn_name {
				b"callBack" => {
					self.callback();
					return true;
				},
				b"version" => {
					self.call_version();
					true
				},
				_other => false,
			} {
				return true;
			}
			false
		}
	}
	pub struct AbiProvider {}

	impl dharitri_wasm::api::ContractAbiProvider for AbiProvider {
		type BigUint = dharitri_wasm::api::uncallable::BigUintUncallable;
		type BigInt = dharitri_wasm::api::uncallable::BigIntUncallable;
		type Storage = dharitri_wasm::api::uncallable::UncallableApi;
		type SendApi = dharitri_wasm::api::uncallable::UncallableApi;

		fn abi() -> dharitri_wasm::abi::ContractAbi {
			let mut contract_abi = dharitri_wasm :: abi :: ContractAbi { docs : & [ "One of the simplest smart contracts possible," , "it holds a single variable in storage, which anyone can increment." ] , name : "Adder" , constructor : None , endpoints : Vec :: new ( ) , type_descriptions : < dharitri_wasm :: abi :: TypeDescriptionContainerImpl as dharitri_wasm :: abi :: TypeDescriptionContainer > :: new ( ) , } ;
			let mut endpoint_abi = dharitri_wasm::abi::EndpointAbi {
				docs: &[],
				name: "version",
				payable_in_tokens: &[],
				inputs: Vec::new(),
				outputs: Vec::new(),
			};
			endpoint_abi.add_output::<Self::BigInt>(&[]);
			contract_abi.add_type_descriptions::<Self::BigInt>();
			contract_abi.endpoints.push(endpoint_abi);
			contract_abi
		}
	}

	pub trait ProxyTrait: dharitri_wasm::api::ProxyObjApi + Sized {
		fn version(
			self,
		) -> ContractCall<Self::SendApi, <Self::BigInt as dharitri_wasm::io::EndpointResult>::DecodeAs>
		{
			let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
				self.into_fields();
			let mut ___contract_call___ = dharitri_wasm::types::new_contract_call(
				___api___.clone(),
				___address___,
				___token___,
				___payment___,
				___nonce___,
				dharitri_wasm::types::BoxedBytes::from(&b"version"[..]),
			);
			___contract_call___
		}
	}
}

mod sample_adder {
	dharitri_wasm::imports!();

	/////////////////////////////////////////////////////////////////////////////////////////////////
	//////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////////////////////////////////////////
	pub trait Adder:
		super::module_1::VersionModule + dharitri_wasm::api::ContractBase + Sized
	where
		Self::BigInt: dharitri_wasm::api::BigIntApi,
		for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
		for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
	{
		fn init(&self, initial_value: &Self::BigInt) {
			self.set_sum(initial_value);
		}
		fn add(&self, value: Self::BigInt) -> SCResult<()> {
			let mut sum = self.get_sum();
			sum.add_assign(value);
			self.set_sum(&sum);
			Ok(())
		}
		fn get_sum(&self) -> Self::BigInt;
		fn set_sum(&self, sum: &Self::BigInt);
		fn add_version(&self) -> SCResult<()> {
			self.add(self.version())
		}
		fn callback(&self);
		fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi>;
	}

	/////////////////////////////////////////////////////////////////////////////////////////////////
	//////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////////////////////////////////////////
	pub trait AutoImpl: dharitri_wasm::api::ContractBase {}

	// impl<C> super::module_1::AutoImpl for C where C: AutoImpl {}

	impl<C> Adder for C
	where
		C: AutoImpl + super::module_1::AutoImpl,
		for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
		for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
	{
		fn get_sum(&self) -> Self::BigInt {
			let key: &'static [u8] = b"sum";
			dharitri_wasm::storage_get(self.get_storage_raw(), &key[..])
		}
		fn set_sum(&self, sum: &Self::BigInt) {
			let key: &'static [u8] = b"sum";
			dharitri_wasm::storage_set(self.get_storage_raw(), &key[..], &sum);
		}
		fn callback(&self) {}
		fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi> {
			<self::CallbackProxyObj::<Self::SendApi> as dharitri_wasm::api::CallbackProxyObjApi>::new_cb_proxy_obj(self.send())
		}
	}

	pub trait EndpointWrappers:
		Adder + dharitri_wasm::api::ContractPrivateApi + super::module_1::EndpointWrappers
	where
		for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
		for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
	{
		#[inline]
		fn call_get_sum(&self) {
			self.call_value().check_not_payable();
			dharitri_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 0i32);
			let result = self.get_sum();
			dharitri_wasm::io::EndpointResult::finish(&result, self.finish_api());
		}
		#[inline]
		fn call_init(&self) {
			self.call_value().check_not_payable();
			dharitri_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
			let initial_value = dharitri_wasm::load_single_arg::<Self::ArgumentApi, Self::BigInt>(
				self.argument_api(),
				0i32,
				ArgId::from(&b"initial_value"[..]),
			);
			self.init(&initial_value);
		}
		#[inline]
		fn call_add(&self) {
			self.call_value().check_not_payable();
			dharitri_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
			let value = dharitri_wasm::load_single_arg::<Self::ArgumentApi, Self::BigInt>(
				self.argument_api(),
				0i32,
				ArgId::from(&b"value"[..]),
			);
			let result = self.add(value);
			dharitri_wasm::io::EndpointResult::finish(&result, self.finish_api());
		}

		fn call(&self, fn_name: &[u8]) -> bool {
			if match fn_name {
				b"callBack" => {
					Adder::callback(self);
					return true;
				},
				[103u8, 101u8, 116u8, 83u8, 117u8, 109u8] => {
					self.call_get_sum();
					true
				},
				[105u8, 110u8, 105u8, 116u8] => {
					self.call_init();
					true
				},
				[97u8, 100u8, 100u8] => {
					self.call_add();
					true
				},
				_other => false,
			} {
				return true;
			}
			if super::module_1::EndpointWrappers::call(self, fn_name) {
				return true;
			}
			false
		}
	}

	pub trait ProxyTrait: dharitri_wasm::api::ProxyObjApi + super::module_1::ProxyTrait {
		fn get_sum(
			self,
		) -> dharitri_wasm::types::ContractCall<
			Self::SendApi,
			<Self::BigInt as dharitri_wasm::io::EndpointResult>::DecodeAs,
		> {
			let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
				self.into_fields();
			let mut ___contract_call___ = dharitri_wasm::types::new_contract_call(
				___api___.clone(),
				___address___,
				___token___,
				___payment___,
				___nonce___,
				dharitri_wasm::types::BoxedBytes::from(&b"get_sum"[..]),
			);
			___contract_call___
		}
		fn add(
			self,
			amount: &Self::BigInt,
		) -> ContractCall<Self::SendApi, <SCResult<()> as dharitri_wasm::io::EndpointResult>::DecodeAs>
		{
			let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
				self.into_fields();
			let mut ___contract_call___ = dharitri_wasm::types::new_contract_call(
				___api___.clone(),
				___address___,
				___token___,
				___payment___,
				___nonce___,
				dharitri_wasm::types::BoxedBytes::from(&b"add"[..]),
			);
			dharitri_wasm::io::serialize_contract_call_arg(
				amount,
				___contract_call___.get_mut_arg_buffer(),
				___api___.clone(),
			);
			___contract_call___
		}
	}

	/////////////////////////////////////////////////////////////////////////////////////////////////
	//////// CONTRACT OBJECT ////////////////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////////////////////////////////////////
	pub struct ContractObj<A: dharitri_wasm::api::ContractBase> {
		api: A,
	}

	/////////////////////////////////////////////////////////////////////////////////////////////////
	//////// CONTRACT OBJECT as CONTRACT BASE ///////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////////////////////////////////////////
	impl<A> dharitri_wasm::api::ContractBase for ContractObj<A>
	where
		A: dharitri_wasm::api::ContractBase
			+ dharitri_wasm::api::ErrorApi
			+ dharitri_wasm::api::EndpointArgumentApi
			+ dharitri_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static,
	{
		type BigUint = A::BigUint;
		type BigInt = A::BigInt;
		type Storage = A::Storage;
		type CallValue = A::CallValue;
		type SendApi = A::SendApi;
		type BlockchainApi = A::BlockchainApi;
		type CryptoApi = A::CryptoApi;
		type LogApi = A::LogApi;
		type ErrorApi = A::ErrorApi;

		#[inline]
		fn get_storage_raw(&self) -> Self::Storage {
			self.api.get_storage_raw()
		}
		#[inline]
		fn call_value(&self) -> Self::CallValue {
			self.api.call_value()
		}
		#[inline]
		fn send(&self) -> Self::SendApi {
			self.api.send()
		}
		#[inline]
		fn blockchain(&self) -> Self::BlockchainApi {
			self.api.blockchain()
		}
		#[inline]
		fn crypto(&self) -> Self::CryptoApi {
			self.api.crypto()
		}
		#[inline]
		fn log_api_raw(&self) -> Self::LogApi {
			self.api.log_api_raw()
		}
		#[inline]
		fn error_api(&self) -> Self::ErrorApi {
			self.api.error_api()
		}
	}

	impl<A> super::module_1::AutoImpl for ContractObj<A> where
		A: dharitri_wasm::api::ContractBase
			+ dharitri_wasm::api::ErrorApi
			+ dharitri_wasm::api::EndpointArgumentApi
			+ dharitri_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static
	{
	}

	impl<A> AutoImpl for ContractObj<A> where
		A: dharitri_wasm::api::ContractBase
			+ dharitri_wasm::api::ErrorApi
			+ dharitri_wasm::api::EndpointArgumentApi
			+ dharitri_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static
	{
	}

	impl<A> dharitri_wasm::api::ContractPrivateApi for ContractObj<A>
	where
		A: dharitri_wasm::api::ContractBase
			+ dharitri_wasm::api::ErrorApi
			+ dharitri_wasm::api::EndpointArgumentApi
			+ dharitri_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static,
	{
		type ArgumentApi = A;
		type FinishApi = A;

		#[inline]
		fn argument_api(&self) -> Self::ArgumentApi {
			self.api.clone()
		}

		#[inline]
		fn finish_api(&self) -> Self::FinishApi {
			self.api.clone()
		}
	}

	impl<A> super::module_1::EndpointWrappers for ContractObj<A>
	where
		A: dharitri_wasm::api::ContractBase
			+ dharitri_wasm::api::ErrorApi
			+ dharitri_wasm::api::EndpointArgumentApi
			+ dharitri_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static,
		for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
		for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
	{
	}

	impl<A> EndpointWrappers for ContractObj<A>
	where
		A: dharitri_wasm::api::ContractBase
			+ dharitri_wasm::api::ErrorApi
			+ dharitri_wasm::api::EndpointArgumentApi
			+ dharitri_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static,
		for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
		for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
		for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
		for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
		for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
		for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
	{
	}

	impl<A> dharitri_wasm::api::CallableContract<A> for ContractObj<A>
	where
		A: dharitri_wasm::api::ContractBase
			+ dharitri_wasm::api::ErrorApi
			+ dharitri_wasm::api::EndpointArgumentApi
			+ dharitri_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static,
		for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
		for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
		for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
		for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
		for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
		for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
		for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
		for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
		for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
		for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
		for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
		for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
		for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
		for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
		for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
		for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
		for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
		for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
		for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
		for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
	{
		fn call(&self, fn_name: &[u8]) -> bool {
			EndpointWrappers::call(self, fn_name)
		}
		fn into_api(self: Box<Self>) -> A {
			self.api
		}
	}

	pub struct AbiProvider {}

	impl dharitri_wasm::api::ContractAbiProvider for AbiProvider {
		type BigUint = dharitri_wasm::api::uncallable::BigUintUncallable;
		type BigInt = dharitri_wasm::api::uncallable::BigIntUncallable;
		type Storage = dharitri_wasm::api::uncallable::UncallableApi;
		type SendApi = dharitri_wasm::api::uncallable::UncallableApi;

		fn abi() -> dharitri_wasm::abi::ContractAbi {
			let mut contract_abi = dharitri_wasm :: abi :: ContractAbi { docs : & [ "One of the simplest smart contracts possible," , "it holds a single variable in storage, which anyone can increment." ] , name : "Adder" , constructor : None , endpoints : Vec :: new ( ) , type_descriptions : < dharitri_wasm :: abi :: TypeDescriptionContainerImpl as dharitri_wasm :: abi :: TypeDescriptionContainer > :: new ( ) , } ;
			let mut endpoint_abi = dharitri_wasm::abi::EndpointAbi {
				docs: &[],
				name: "getSum",
				payable_in_tokens: &[],
				inputs: Vec::new(),
				outputs: Vec::new(),
			};
			endpoint_abi.add_output::<Self::BigInt>(&[]);
			contract_abi.add_type_descriptions::<Self::BigInt>();
			contract_abi.endpoints.push(endpoint_abi);
			let mut endpoint_abi = dharitri_wasm::abi::EndpointAbi {
				docs: &[],
				name: "init",
				payable_in_tokens: &[],
				inputs: Vec::new(),
				outputs: Vec::new(),
			};
			endpoint_abi.add_input::<&Self::BigInt>("initial_value");
			contract_abi.add_type_descriptions::<&Self::BigInt>();
			contract_abi.constructor = Some(endpoint_abi);
			let mut endpoint_abi = dharitri_wasm::abi::EndpointAbi {
				docs: &["Add desired amount to the storage variable."],
				name: "add",
				payable_in_tokens: &[],
				inputs: Vec::new(),
				outputs: Vec::new(),
			};
			endpoint_abi.add_input::<&Self::BigInt>("value");
			contract_abi.add_type_descriptions::<&Self::BigInt>();
			endpoint_abi.add_output::<SCResult<()>>(&[]);
			contract_abi.add_type_descriptions::<SCResult<()>>();
			contract_abi.endpoints.push(endpoint_abi);
			contract_abi.coalesce(
				<super::module_1::AbiProvider as dharitri_wasm::api::ContractAbiProvider>::abi(),
			);
			contract_abi
		}
	}

	pub fn contract_obj<A>(api: A) -> ContractObj<A>
	where
		A: dharitri_wasm::api::ContractBase
			+ dharitri_wasm::api::ErrorApi
			+ dharitri_wasm::api::EndpointArgumentApi
			+ dharitri_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static,
		for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
		for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
		for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
		for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
		for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
		for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
		for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
		for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
		for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
		for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
		for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
		for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
		for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
		for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
		for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
		for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
		for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
		for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
		for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
		for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
		for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
	{
		ContractObj { api }
	}

	pub struct Proxy<SA>
	where
		SA: dharitri_wasm::api::SendApi + 'static,
	{
		pub api: SA,
		pub address: Address,
		pub payment_token: dharitri_wasm::types::TokenIdentifier,
		pub payment_amount: SA::AmountType,
		pub payment_nonce: u64,
	}

	impl<SA> dharitri_wasm::api::ProxyObjApi for Proxy<SA>
	where
		SA: dharitri_wasm::api::SendApi + 'static,
	{
		type BigUint = SA::AmountType;
		type BigInt = SA::ProxyBigInt;
		type Storage = SA::ProxyStorage;
		type SendApi = SA;

		fn new_proxy_obj(api: SA, address: Address) -> Self {
			Proxy {
				api,
				address,
				payment_token: dharitri_wasm::types::TokenIdentifier::moax(),
				payment_amount: Self::BigUint::zero(),
				payment_nonce: 0,
			}
		}

		fn with_token_transfer(mut self, token: TokenIdentifier, payment: Self::BigUint) -> Self {
			self.payment_token = token;
			self.payment_amount = payment;
			self
		}

		#[inline]
		fn with_nft_nonce(mut self, nonce: u64) -> Self {
			self.payment_nonce = nonce;
			self
		}

		#[inline]
		fn into_fields(self) -> (Self::SendApi, Address, TokenIdentifier, Self::BigUint, u64) {
			(
				self.api,
				self.address,
				self.payment_token,
				self.payment_amount,
				self.payment_nonce,
			)
		}
	}

	impl<SA> super::module_1::ProxyTrait for Proxy<SA> where SA: dharitri_wasm::api::SendApi {}

	impl<SA> ProxyTrait for Proxy<SA> where SA: dharitri_wasm::api::SendApi {}

	pub fn new_proxy_obj<SA>(api: SA, address: Address) -> impl ProxyTrait
	where
		SA: dharitri_wasm::api::SendApi + 'static,
	{
		Proxy::new_proxy_obj(api, address)
	}

	pub struct CallbackProxyObj<SA>
	where
		SA: dharitri_wasm::api::SendApi + 'static,
	{
		pub api: SA,
	}
	impl<SA> dharitri_wasm::api::CallbackProxyObjApi for CallbackProxyObj<SA>
	where
		SA: dharitri_wasm::api::SendApi + 'static,
	{
		type BigUint = SA::AmountType;
		type BigInt = SA::ProxyBigInt;
		type Storage = SA::ProxyStorage;
		type SendApi = SA;
		type ErrorApi = SA;
		fn new_cb_proxy_obj(api: SA) -> Self {
			CallbackProxyObj { api }
		}
		fn into_api(self) -> Self::ErrorApi {
			self.api
		}
	}

	pub trait CallbackProxy: dharitri_wasm::api::CallbackProxyObjApi + Sized {
		fn my_callback(self, caller: &Address) -> dharitri_wasm::types::CallbackCall {
			let ___api___ = self.into_api();
			let mut ___closure_arg_buffer___ = dharitri_wasm::types::ArgBuffer::new();
			dharitri_wasm::io::serialize_contract_call_arg(
				caller,
				&mut ___closure_arg_buffer___,
				___api___.clone(),
			);
			dharitri_wasm::types::CallbackCall::from_arg_buffer(
				&b"my_callback"[..],
				&___closure_arg_buffer___,
			)
		}
	}
	impl<SA> self::CallbackProxy for CallbackProxyObj<SA> where SA: dharitri_wasm::api::SendApi + 'static {}
}

#[test]
fn test_add() {
	use dharitri_wasm::api::ContractBase;
	use dharitri_wasm_debug::api::RustBigInt;
	use dharitri_wasm_debug::TxContext;
	use sample_adder::{Adder, EndpointWrappers, ProxyTrait};
	// use module_1::{VersionModule, EndpointWrappers};

	let tx_context = TxContext::dummy();

	let adder = sample_adder::contract_obj(tx_context.clone());

	adder.init(&RustBigInt::from(5));
	assert_eq!(RustBigInt::from(5), adder.get_sum());

	let _ = adder.add(RustBigInt::from(7));
	assert_eq!(RustBigInt::from(12), adder.get_sum());

	let _ = adder.add(RustBigInt::from(-1));
	assert_eq!(RustBigInt::from(11), adder.get_sum());

	assert_eq!(RustBigInt::from(100), adder.version());

	let _ = adder.add_version();
	assert_eq!(RustBigInt::from(111), adder.get_sum());

	assert!(!adder.call(b"invalid_endpoint"));

	assert!(adder.call(b"version"));

	let own_proxy = sample_adder::new_proxy_obj(adder.send(), Address::zero());
	let _ = own_proxy.get_sum();

	let _ = dharitri_wasm_debug::abi_json::contract_abi::<sample_adder::AbiProvider>();
}

fn contract_map() -> dharitri_wasm_debug::ContractMap<dharitri_wasm_debug::TxContext> {
	let mut contract_map = dharitri_wasm_debug::ContractMap::new();
	contract_map.register_contract(
		"file:../output/adder.wasm",
		Box::new(|context| Box::new(sample_adder::contract_obj(context))),
	);
	contract_map
}

#[test]
fn test_denali() {
	dharitri_wasm_debug::denali_rs(
		"../contracts/examples/adder/denali/adder.scen.json",
		&contract_map(),
	);
}
