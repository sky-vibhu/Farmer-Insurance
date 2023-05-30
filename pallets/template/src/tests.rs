use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
// use crate::mock::RuntimeOrigin;



 	#[test]
 	fn register_farmer_should_work() {
 		new_test_ext().execute_with(|| {
             
             assert_ok!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 123456789021, 100000));
			 System::assert_last_event(Event::NewFarmerStored(0, 123456789021, 100000, 2).into());
             assert_noop!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 123456789021, 100000), Error::<Test>::FarmerAlreadyExists);
             assert_noop!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 12345678902, 100000), Error::<Test>::AadhaarNumberNotCorrect);

             assert_ok!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 567890123456, 100000));
             assert_noop!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 567890123456, 100000), Error::<Test>::FarmerAlreadyExists);
             assert_noop!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 56789012345, 100000), Error::<Test>::AadhaarNumberNotCorrect);

             assert_ok!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 678901234565, 1000));
             assert_noop!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 678901234565, 1000), Error::<Test>::FarmerAlreadyExists);
             assert_noop!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 67890123456, 1000), Error::<Test>::AadhaarNumberNotCorrect);

             assert_ok!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 567890123454, 1000));
             assert_noop!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 567890123454, 1000), Error::<Test>::FarmerAlreadyExists);
             assert_noop!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 56789012345, 1000), Error::<Test>::AadhaarNumberNotCorrect);

             assert_ok!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 567890123457, 1000));
             assert_noop!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 567890123457, 1000), Error::<Test>::FarmerAlreadyExists);
             assert_noop!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 56789012345, 1000), Error::<Test>::AadhaarNumberNotCorrect);

 			
 		});
 	}

    // Insurance will only be registered after registering farmer first.
    #[test]
     fn register_insurance_should_work() {
        new_test_ext().execute_with(|| {

            // FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 123456789012, 10, 100000, 12345678);
            // FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 123456789012, 123);
            // FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 123456789012, 10, 100000, 12345678);
            // FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 12345678901, 10, 100000, 12345678);


            assert_noop!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 345678901234, 10, 100000, 23456789), Error::<Test>::FarmerNotRegistered);
            assert_ok!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 345678901234, 123));
            assert_ok!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 345678901234, 10, 100000, 23456789));
            assert_noop!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 34567890123, 10, 100000, 23456789), Error::<Test>::AadhaarNumberNotCorrect);

            assert_noop!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 456789012345, 10, 100000, 23456789), Error::<Test>::FarmerNotRegistered);
            assert_ok!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 456789012345, 12));
            assert_ok!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 456789012345, 10, 100000, 23456789));
            assert_noop!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 45678901234, 10, 100000, 23456789), Error::<Test>::AadhaarNumberNotCorrect);

            assert_noop!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 456789012346, 10, 100000, 23456789), Error::<Test>::FarmerNotRegistered);
            assert_ok!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 456789012346, 12));
            assert_ok!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 456789012346, 10, 100000, 23456789));
            assert_noop!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 45678901234, 10, 100000, 23456789), Error::<Test>::AadhaarNumberNotCorrect);

           
            assert_noop!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 456789012347, 10, 100000, 23456789), Error::<Test>::FarmerNotRegistered);
            assert_ok!(FarmInsurance::register_farmer(RuntimeOrigin::signed(2), 456789012347, 12));
            assert_ok!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 456789012347, 10, 100000, 23456789));
            assert_noop!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 45678901234, 10, 100000, 23456789), Error::<Test>::AadhaarNumberNotCorrect);


        });
    }


    // Insurance will not be registered without registering the farmer first.
    #[test]
     fn register_insurance_should_not_work() {
        new_test_ext().execute_with(|| {
             assert_ok!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 123456789012, 10, 100000, 09876543));
             assert_ok!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 345678901234, 10, 100000, 23456789));
             assert_ok!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 456789012343, 10, 100000, 23456789));
             assert_ok!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 345678901232, 10, 100000, 23456789));
             assert_ok!(FarmInsurance::register_insurance(RuntimeOrigin::signed(2), 456789012347, 10, 100000, 23456789));

        });
    }

    #[test]
    fn query_insurance_validity_should_work() {
       new_test_ext().execute_with(|| {
            assert_ok!(FarmInsurance::query_insurance_validity(RuntimeOrigin::signed(2), 0));
            assert_ok!(FarmInsurance::query_insurance_validity(RuntimeOrigin::signed(2), 1));
            assert_ok!(FarmInsurance::query_insurance_validity(RuntimeOrigin::signed(2), 2));
            assert_ok!(FarmInsurance::query_insurance_validity(RuntimeOrigin::signed(2), 3));
            assert_ok!(FarmInsurance::query_insurance_validity(RuntimeOrigin::signed(2), 4));

       });
   }

   #[test]
    fn query_insurance_amount_should_work() {
       new_test_ext().execute_with(|| {
            assert_ok!(FarmInsurance::query_insurance_amount(RuntimeOrigin::signed(2), 0));
            assert_ok!(FarmInsurance::query_insurance_amount(RuntimeOrigin::signed(2), 1));
            assert_ok!(FarmInsurance::query_insurance_amount(RuntimeOrigin::signed(2), 2));
            assert_ok!(FarmInsurance::query_insurance_amount(RuntimeOrigin::signed(2), 3));
            assert_ok!(FarmInsurance::query_insurance_amount(RuntimeOrigin::signed(2), 4));
       });
   }

   #[test]
    fn query_insurance_land_co_ordinates_should_work() {
       new_test_ext().execute_with(|| {
            assert_ok!(FarmInsurance::query_insurance_land_co_ordinates(RuntimeOrigin::signed(2), 0));
            assert_ok!(FarmInsurance::query_insurance_land_co_ordinates(RuntimeOrigin::signed(2), 1));
            assert_ok!(FarmInsurance::query_insurance_land_co_ordinates(RuntimeOrigin::signed(2), 2));
            assert_ok!(FarmInsurance::query_insurance_land_co_ordinates(RuntimeOrigin::signed(2), 3));
            assert_ok!(FarmInsurance::query_insurance_land_co_ordinates(RuntimeOrigin::signed(2), 4));

       });
   }

   #[test]
    fn query_farmer_index_should_work() {
       new_test_ext().execute_with(|| {
        assert_ok!(FarmInsurance::query_farmer_index(RuntimeOrigin::signed(2), 123456789012));
        assert_ok!(FarmInsurance::query_farmer_index(RuntimeOrigin::signed(2), 123456789011));
        assert_ok!(FarmInsurance::query_farmer_index(RuntimeOrigin::signed(2), 123456789013));
        assert_ok!(FarmInsurance::query_farmer_index(RuntimeOrigin::signed(2), 123456789014));
        assert_ok!(FarmInsurance::query_farmer_index(RuntimeOrigin::signed(2), 123456789015));
            
       });
   }


