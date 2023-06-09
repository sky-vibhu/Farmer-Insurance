
//! Autogenerated weights for `pallet_farm_insurance`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ant-ThinkPad-E14-Gen-2`, CPU: `11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_farm_insurance
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/transfer-weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_farm_insurance`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_farm_insurance::WeightInfo for WeightInfo<T> {
	/// Storage: FarmInsurance AadhaarIndex (r:1 w:1)
	/// Proof: FarmInsurance AadhaarIndex (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	/// Storage: FarmInsurance TotalRegisteredFarmer (r:1 w:1)
	/// Proof: FarmInsurance TotalRegisteredFarmer (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: FarmInsurance FarmerOwnedLand (r:0 w:1)
	/// Proof: FarmInsurance FarmerOwnedLand (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: FarmInsurance FarmerAadharDetails (r:0 w:1)
	/// Proof: FarmInsurance FarmerAadharDetails (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn register_farmer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `4982`
		// Minimum execution time: 21_071_000 picoseconds.
		Weight::from_parts(21_419_000, 0)
			.saturating_add(Weight::from_parts(0, 4982))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FarmInsurance AadhaarIndex (r:1 w:0)
	/// Proof: FarmInsurance AadhaarIndex (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	/// Storage: FarmInsurance TotalRegisteredInsurance (r:1 w:1)
	/// Proof: FarmInsurance TotalRegisteredInsurance (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: FarmInsurance Certificate (r:1 w:1)
	/// Proof: FarmInsurance Certificate (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: FarmInsurance InsuranceValidity (r:0 w:1)
	/// Proof: FarmInsurance InsuranceValidity (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: FarmInsurance LandCoOrdinates (r:0 w:1)
	/// Proof: FarmInsurance LandCoOrdinates (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: FarmInsurance InsuranceAmount (r:0 w:1)
	/// Proof: FarmInsurance InsuranceAmount (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn register_insurance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `148`
		//  Estimated: `8471`
		// Minimum execution time: 25_107_000 picoseconds.
		Weight::from_parts(28_746_000, 0)
			.saturating_add(Weight::from_parts(0, 8471))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: FarmInsurance InsuranceValidity (r:1 w:0)
	/// Proof: FarmInsurance InsuranceValidity (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn query_insurance_validity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3489`
		// Minimum execution time: 14_401_000 picoseconds.
		Weight::from_parts(14_987_000, 0)
			.saturating_add(Weight::from_parts(0, 3489))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: FarmInsurance InsuranceAmount (r:1 w:0)
	/// Proof: FarmInsurance InsuranceAmount (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn query_insurance_amount() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `243`
		//  Estimated: `3489`
		// Minimum execution time: 14_194_000 picoseconds.
		Weight::from_parts(14_641_000, 0)
			.saturating_add(Weight::from_parts(0, 3489))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: FarmInsurance LandCoOrdinates (r:1 w:0)
	/// Proof: FarmInsurance LandCoOrdinates (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn query_insurance_land_co_ordinates() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3489`
		// Minimum execution time: 14_448_000 picoseconds.
		Weight::from_parts(14_910_000, 0)
			.saturating_add(Weight::from_parts(0, 3489))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: FarmInsurance AadhaarIndex (r:1 w:0)
	/// Proof: FarmInsurance AadhaarIndex (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn query_farmer_index() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `148`
		//  Estimated: `3493`
		// Minimum execution time: 16_123_000 picoseconds.
		Weight::from_parts(16_520_000, 0)
			.saturating_add(Weight::from_parts(0, 3493))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}
