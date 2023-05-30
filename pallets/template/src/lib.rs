#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
use frame_support::pallet_prelude::Weight;
pub use weights::FarmWeightInfo;
pub trait WeightInfo {
	fn register_farmer() -> Weight;
	fn register_insurance() -> Weight;
	fn query_insurance_validity() -> Weight;
	fn query_insurance_amount() -> Weight;
	fn query_insurance_land_co_ordinates() -> Weight;
	fn query_farmer_index() -> Weight;
}
	
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);


	#[pallet::config]
	pub trait Config: frame_system::Config {
 		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		 type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn a)]
    pub type TotalRegisteredFarmer<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn b)]
    pub type TotalRegisteredInsurance<T> = StorageValue<_, u32, ValueQuery>;


    #[pallet::storage]
    #[pallet::getter(fn c)]
    pub(super) type FarmerInsurance<T: Config> = StorageMap< _, 
    Blake2_128Concat, 
    u32, 
    u32, 
    ValueQuery>;


    #[pallet::storage]
    #[pallet::getter(fn d)]
    pub(super) type InsuranceValidity<T: Config> = StorageMap<
        _, 
        Blake2_128Concat,
        u32, 
        u32, 
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn e)]
    pub(super) type InsuranceAmount<T: Config> = StorageMap<
        _, 
        Blake2_128Concat,
        u32, 
        u32, 
        ValueQuery
    >;   

    #[pallet::storage]
    #[pallet::getter(fn farmers)]
    // #[scale_info(skip_type_params(T))]
    pub(super) type FarmerAadharDetails<T: Config> = StorageMap<
        _, 
        Blake2_128Concat,
        u32, 
        u64,  
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn g)]
    // #[scale_info(skip_type_params(T))]
    pub(super) type AadhaarIndex<T: Config> = StorageMap<
        _, 
        Blake2_128Concat,
        u64, 
        u32,  
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn h)]
    pub(super) type AadhaarCertificate<T: Config> = StorageMap<
        _, 
        Blake2_128Concat,
        u64, 
        u32,  
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn i)]
    pub(super) type FarmerOwnedLand<T: Config> = StorageMap<
        _, 
        Blake2_128Concat,
        u32, 
        u32, 
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn j)]
    pub(super) type Certificate<T: Config> = StorageMap<
        _, 
        Blake2_128Concat,
        u32, 
        u32, 
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn k)]
    pub(super) type LandCoOrdinates<T: Config> = StorageMap<
        _, 
        Blake2_128Concat,
        u32, 
        u32, 
        ValueQuery
    >;
	  
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        NewFarmerStored(
            u32, 
            u64,
            u32,
            T::AccountId),
        
        NewInsuranceStored(
            u32, 
            u32, 
            u32, 
            u32, 
            T::AccountId),

        QueryInsuranceValidity(
            u32,
            u32,
            T::AccountId
        ),
        QueryInsuranceAmount(
            u32,
            u32,
            T::AccountId
        ),
        QueryInsuranceLand(
            u32,
            u32,
            T::AccountId
        ),
        QueryFarmerId(
            u64,
            u32,
            T::AccountId
        ),
        QueryInsuranceCertificate(
            u64,
            u32,
            T::AccountId
        ),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
        FarmerNotRegistered,
        AadhaarNumberNotCorrect, 
        FarmerAlreadyExists,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::register_farmer())]
		pub fn register_farmer(
            origin: OriginFor<T>, 
            aadhaar_number: u64, 
            land_owned: u32,) -> 
            DispatchResult {
			
			let who = ensure_signed(origin)?;

            ensure!(!Self::already_registered_farmer(
                aadhaar_number), 
                 Error::<T>::FarmerAlreadyExists);

            ensure!(!Self::correct_aadhaar(
              aadhaar_number), 
             Error::<T>::AadhaarNumberNotCorrect);

            if  !<TotalRegisteredFarmer<T>>::exists() { <TotalRegisteredFarmer<T>>::put(0u32);}
            else{<TotalRegisteredFarmer<T>>::mutate(|farmer_index_id| *farmer_index_id+= 1);}

            let farmer_index_id =  <TotalRegisteredFarmer<T>>::get();
                <FarmerAadharDetails<T>>::insert(
                    farmer_index_id,
                    aadhaar_number,
                );

                <AadhaarIndex<T>>::insert(
                    aadhaar_number,
                    farmer_index_id,
                );

                <FarmerOwnedLand<T>>::insert(
                    farmer_index_id,
                    land_owned,
                );


			Self::deposit_event(Event::NewFarmerStored(
                farmer_index_id, 
                aadhaar_number, 
                land_owned, 
                who));
			Ok(())
		}

        #[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::register_insurance())]
		pub fn register_insurance(
            origin: OriginFor<T>, 
            aadhaar_number: u64,
            validity: u32, 
            amount: u32,
            land_co_ordinates: u32 ) ->
             DispatchResult {
			
			let who = ensure_signed(origin)?;
			
            ensure!(!Self::correct_aadhaar(
                aadhaar_number), 
                 Error::<T>::AadhaarNumberNotCorrect);

            ensure!(!Self::registered_farmer(
                aadhaar_number), 
                 Error::<T>::FarmerNotRegistered);

            let farmer_index_id = <AadhaarIndex<T>>::get(aadhaar_number);

            if  !<TotalRegisteredInsurance<T>>::exists() { <TotalRegisteredInsurance<T>>::put(0u32);}
            else{<TotalRegisteredInsurance<T>>::mutate(|val| *val+= 1);}
                      
            let certificate_number =  <Certificate<T>>::get(farmer_index_id);
            <Certificate<T>>::insert(
                farmer_index_id,
                certificate_number,
            );

            <InsuranceValidity<T>>::insert(
                certificate_number,
                validity,
            );

            <InsuranceAmount<T>>::insert(
                certificate_number,
                amount,
            );

             <LandCoOrdinates<T>>::insert(
                    farmer_index_id,
                    land_co_ordinates.clone(),
                );

            
			Self::deposit_event(Event::NewInsuranceStored(
                farmer_index_id, 
                certificate_number,
                validity,
                amount,
                who ));
			Ok(())
		}

        #[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::query_insurance_validity())]
			pub fn query_insurance_validity(
            origin: OriginFor<T>,
            certificate_number: u32,
        ) -> DispatchResult
        {
            let who = ensure_signed(origin)?;
            let validity = <InsuranceValidity<T>>::get(certificate_number.clone());
            Self::deposit_event(Event::QueryInsuranceValidity(
                certificate_number.clone(),
                validity,
                who,
            ));
            Ok(())
        }

        #[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::query_insurance_amount())]
			pub fn query_insurance_amount(
            origin: OriginFor<T>,
            certificate_number: u32,
        ) -> DispatchResult
        {
            let who = ensure_signed(origin)?;
            let amount = <InsuranceAmount<T>>::get(certificate_number.clone());
            Self::deposit_event(Event::QueryInsuranceAmount(
                certificate_number.clone(),
                amount,
                who,
            ));
            Ok(())
          }

          #[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::query_insurance_land_co_ordinates())]          
		pub fn query_insurance_land_co_ordinates(
              origin: OriginFor<T>,
              certificate_number: u32,
          ) -> DispatchResult
          {
              let who = ensure_signed(origin)?;
              let land = <LandCoOrdinates<T>>::get(certificate_number.clone());
              Self::deposit_event(Event::QueryInsuranceLand(
                  certificate_number.clone(),
                  land,
                  who,
              ));
              Ok(())
            }

            #[pallet::call_index(5)]
			#[pallet::weight(T::WeightInfo::query_farmer_index())]
            pub fn query_farmer_index(
                origin: OriginFor<T>,
                aadhaar_number: u64,
            ) -> DispatchResult
            {
                let who = ensure_signed(origin)?;
                let farmer_index = <AadhaarIndex<T>>::get(aadhaar_number);
                Self::deposit_event(Event::QueryFarmerId(
                    aadhaar_number,
                    farmer_index,
                    who,
                ));
                Ok(())
              }
        }
	
    impl<T: Config> Pallet<T> {
    pub fn registered_farmer(aadhaar: u64) -> bool   {
        match <AadhaarIndex<T>>::contains_key(aadhaar) {
            true => false,
            false => true,
        }
    }

    pub fn already_registered_farmer(aadhaar: u64) -> bool   {
        match <AadhaarIndex<T>>::contains_key(aadhaar) {
            true => true,
            false => false,
        }
    }

    pub fn correct_aadhaar(aadhaar: u64) -> bool {
            if aadhaar > 99999999999 && aadhaar < 1000000000000 {
            false
            }
            else  { 
            true
            }
        }
}
} 

