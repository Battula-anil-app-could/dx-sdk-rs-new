#![no_std]

mod internal_mod_a;
mod internal_mod_b;
mod internal_mod_c;

dharitri_wasm::imports!();

#[cfg(feature = "dharitri-wasm-module-dns-default")]
pub use dharitri_wasm_module_dns_default as dns;
#[cfg(feature = "dharitri-wasm-module-dns-wasm")]
pub use dharitri_wasm_module_dns_wasm as dns;

#[cfg(feature = "dharitri-wasm-module-features-default")]
pub use dharitri_wasm_module_features_default as features;
#[cfg(feature = "dharitri-wasm-module-features-wasm")]
pub use dharitri_wasm_module_features_wasm as features;

#[cfg(feature = "dharitri-wasm-module-pause-default")]
pub use dharitri_wasm_module_pause_default as pause;
#[cfg(feature = "dharitri-wasm-module-pause-wasm")]
pub use dharitri_wasm_module_pause_wasm as pause;

use features::feature_guard;

/// Contract that tests that using modules works correctly.
/// Also provides testing for the most common modules:
/// - DnsModule
/// - FeaturesModule
/// - PauseModule
#[dharitri_wasm_derive::contract]
pub trait UseModule:
	internal_mod_a::InternalModuleA
	+ internal_mod_b::InternalModuleB
	+ internal_mod_c::InternalModuleC
	+ features::FeaturesModule
	+ pause::PauseModule
	+ dns::DnsModule
{
	#[init]
	fn init(&self) {}

	/// Validates that the "featureName" feature is on.
	/// Uses the `feature_guard!` macro.
	#[endpoint(checkFeatureGuard)]
	fn check_feature_guard(&self) -> SCResult<()> {
		feature_guard!(self, b"featureName", true);
		Ok(())
	}

	#[endpoint(checkPause)]
	fn check_pause(&self) -> SCResult<bool> {
		Ok(self.is_paused())
	}
}
