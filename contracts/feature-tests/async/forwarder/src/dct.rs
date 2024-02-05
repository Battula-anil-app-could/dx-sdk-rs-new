dharitri_wasm::imports!();

use super::storage::*;

#[dharitri_wasm_derive::module(ForwarderDctModuleImpl)]
pub trait ForwarderDctModule {
	#[module(ForwarderStorageModuleImpl)]
	fn storage_module(&self) -> ForwarderStorageModuleImpl<T, BigInt, BigUint>;

	#[view(getFungibleDctBalance)]
	fn get_fungible_dct_balance(&self, token_identifier: &TokenIdentifier) -> BigUint {
		self.blockchain().get_dct_balance(
			&self.blockchain().get_sc_address(),
			token_identifier.as_dct_identifier(),
			0,
		)
	}

	#[endpoint]
	fn send_dct(
		&self,
		to: &Address,
		token_id: BoxedBytes,
		amount: &BigUint,
		#[var_args] opt_data: OptionalArg<BoxedBytes>,
	) {
		let data = match &opt_data {
			OptionalArg::Some(data) => data.as_slice(),
			OptionalArg::None => &[],
		};
		let _ = self
			.send()
			.direct_dct_via_transf_exec(to, token_id.as_slice(), amount, data);
	}

	#[endpoint]
	fn send_dct_twice(
		&self,
		to: &Address,
		token_id: BoxedBytes,
		amount_first_time: &BigUint,
		amount_second_time: &BigUint,
		#[var_args] opt_data: OptionalArg<BoxedBytes>,
	) {
		let data = match &opt_data {
			OptionalArg::Some(data) => data.as_slice(),
			OptionalArg::None => &[],
		};
		let _ = self.send().direct_dct_via_transf_exec(
			to,
			token_id.as_slice(),
			amount_first_time,
			data,
		);
		let _ = self.send().direct_dct_via_transf_exec(
			to,
			token_id.as_slice(),
			amount_second_time,
			data,
		);
	}

	#[payable("MOAX")]
	#[endpoint]
	fn issue_fungible_token(
		&self,
		#[payment] issue_cost: BigUint,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: BigUint,
	) -> AsyncCall<BigUint> {
		let caller = self.blockchain().get_caller();

		DCTSystemSmartContractProxy::new()
			.issue_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				&initial_supply,
				FungibleTokenProperties {
					num_decimals: 0,
					can_freeze: true,
					can_wipe: true,
					can_pause: true,
					can_mint: true,
					can_burn: true,
					can_change_owner: true,
					can_upgrade: true,
					can_add_special_roles: true,
				},
			)
			.async_call()
			.with_callback(self.callbacks().dct_issue_callback(&caller))
	}

	#[callback]
	fn dct_issue_callback(
		&self,
		caller: &Address,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment] returned_tokens: BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) {
		// callback is called with DCTTransfer of the newly issued token, with the amount requested,
		// so we can get the token identifier and amount from the call data
		match result {
			AsyncCallResult::Ok(()) => {
				self.storage_module()
					.last_issued_token()
					.set(&token_identifier);
				self.storage_module().last_error_message().clear();
			},
			AsyncCallResult::Err(message) => {
				// return issue cost to the caller
				if token_identifier.is_moax() && returned_tokens > 0 {
					self.send().direct_moax(caller, &returned_tokens, &[]);
				}

				self.storage_module()
					.last_error_message()
					.set(&message.err_msg);
			},
		}
	}

	#[endpoint]
	fn local_mint(&self, token_identifier: TokenIdentifier, amount: BigUint) {
		self.send().dct_local_mint(
			self.blockchain().get_gas_left(),
			token_identifier.as_dct_identifier(),
			&amount,
		);
	}

	#[endpoint]
	fn local_burn(&self, token_identifier: TokenIdentifier, amount: BigUint) {
		self.send().dct_local_burn(
			self.blockchain().get_gas_left(),
			token_identifier.as_dct_identifier(),
			&amount,
		);
	}
}
