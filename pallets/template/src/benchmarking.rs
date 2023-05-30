#![cfg(feature = "runtime-benchmarks")]
#[warn(unused_must_use)]

use crate::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;


#[benchmarks]
mod benchmarks{
	
	use super::*;

    fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
		frame_system::Pallet::<T>::assert_last_event(generic_event.into());
	}

    #[benchmark]
    fn register_farmer() {
        let farmer: T::AccountId = account("farmer",1u32,2u32);
        let farmer_origin= RawOrigin::Signed(farmer.clone());
        let adhar= 123456789012u64;
        let land =1231u32;
		let time =2u32;
		frame_system::Pallet::<T>::set_block_number(time.into());

        #[extrinsic_call]
        _(farmer_origin, adhar, land);

        assert_last_event::<T>(Event::NewFarmerStored(0u32, adhar, land, farmer.into()).into());

    }

	#[benchmark]
    fn register_insurance() {
        let farmer: T::AccountId = account("farmer",1u32,2u32);
        let farmer_origin= RawOrigin::Signed(farmer.clone());
        let adhar= 123456789012u64;
		let land =1231u32;
		let validity = 12u32;
		let amount = 100000u32;
        let land_co_ordinates =1231u32;
		// let index_id = 0u32;
		let time =2u32;
		frame_system::Pallet::<T>::set_block_number(time.into());
		
		Pallet::<T>::register_farmer(farmer_origin.clone().into(), adhar, land);


        #[extrinsic_call]
        _(farmer_origin, adhar, validity, amount, land_co_ordinates);

        assert_last_event::<T>(Event::NewInsuranceStored(0u32, 0u32, validity, amount, farmer.into()).into());
    }

	#[benchmark]
    fn query_insurance_validity() {
        let farmer: T::AccountId = account("farmer",1u32,2u32);
        let farmer_origin= RawOrigin::Signed(farmer.clone());
        let adhar= 123456789012u64;
		let land =1231u32;
		let valid = 12u32;
		let amount = 100000u32;
        let land_co_ordinates =1231u32;
		// let index_id = 0u32;
		let time =2u32;
		frame_system::Pallet::<T>::set_block_number(time.into());
		
		Pallet::<T>::register_farmer(farmer_origin.clone().into(), adhar, land);
		Pallet::<T>::register_insurance(farmer_origin.clone().into(), adhar, valid, amount, land_co_ordinates);

        #[extrinsic_call]
        _(farmer_origin.clone(), 0u32);
		let validity = <InsuranceValidity<T>>::get(0u32);

        assert_last_event::<T>(Event::QueryInsuranceValidity(0u32, validity, farmer.into()).into());
    }

	#[benchmark]
    fn query_insurance_amount() {
        let farmer: T::AccountId = account("farmer",1u32,2u32);
        let farmer_origin= RawOrigin::Signed(farmer.clone());
        let adhar= 123456789012u64;
		let land =1231u32;
		let validity = 12u32;
		let amounting = 100000u32;
        let land_co_ordinates =1231u32;
		// let index_id = 0u32;
		let time =2u32;
		frame_system::Pallet::<T>::set_block_number(time.into());
		
		Pallet::<T>::register_farmer(farmer_origin.clone().into(), adhar, land);
		Pallet::<T>::register_insurance(farmer_origin.clone().into(), adhar, validity, amounting, land_co_ordinates);

        #[extrinsic_call]
        _(farmer_origin.clone(), 0u32);
		let amount = <InsuranceAmount<T>>::get(0u32);

        assert_last_event::<T>(Event::QueryInsuranceAmount(0u32, amount, farmer.into()).into());
    }

	#[benchmark]
    fn query_insurance_land_co_ordinates() {
        let farmer: T::AccountId = account("farmer",1u32,2u32);
        let farmer_origin= RawOrigin::Signed(farmer.clone());
        let adhar= 123456789012u64;
		let land =1231u32;
		let validity = 12u32;
		let amounting = 100000u32;
        let land_co_ordinates =1231u32;
		let time =2u32;
		frame_system::Pallet::<T>::set_block_number(time.into());
		
		Pallet::<T>::register_farmer(farmer_origin.clone().into(), adhar, land);
		Pallet::<T>::register_insurance(farmer_origin.clone().into(), adhar, validity, amounting, land_co_ordinates);

        #[extrinsic_call]
        _(farmer_origin.clone(), 0u32);
		let query_land = <LandCoOrdinates<T>>::get(0u32);

        assert_last_event::<T>(Event::QueryInsuranceLand(0u32, query_land, farmer.into()).into());
    }

	#[benchmark]
    fn query_farmer_index() {
        let farmer: T::AccountId = account("farmer",1u32,2u32);
        let farmer_origin= RawOrigin::Signed(farmer.clone());
        let adhar= 123456789012u64;
		let land =1231u32;
		let time =2u32;
		frame_system::Pallet::<T>::set_block_number(time.into());
		
		Pallet::<T>::register_farmer(farmer_origin.clone().into(), adhar, land);

        #[extrinsic_call]
        _(farmer_origin.clone(), adhar);
		let farmer_index = <AadhaarIndex<T>>::get(adhar);

        assert_last_event::<T>(Event::QueryFarmerId(adhar, farmer_index, farmer.into()).into());
    }

	#[cfg(test)]
	mod tests {
	use crate::mock::Test;
	use sp_io::TestExternalities;

	pub fn new_test_ext() -> TestExternalities {
		let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		TestExternalities::new(t)
	}
}

// This line generates test cases for benchmarking, and could be run by:
//   `cargo test -p pallet-example-basic --all-features`, you will see one line per case:
//
// The line generates three steps per benchmark, with repeat=1 and the three steps are
//   [low, mid, high] of the range.
impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);

}

