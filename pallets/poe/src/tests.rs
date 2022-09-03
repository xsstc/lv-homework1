use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok,BoundedVec};
use sp_runtime::AccountId32;

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        let bounded_claim = 
            BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

        assert_eq!(
            Proof::<Test>::get(&bounded_claim),
            Some((1,frame_system::Pallet::<Test>::block_number()))
        );
    })
}

#[test]
fn create_claim_failed_when_claim_already_exsit() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExit
        );
    })
}

#[test]
fn remove_claim_works(){
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        let bounded_claim = 
            BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

        assert_eq!(
            Proof::<Test>::get(&bounded_claim),
            Some((1,frame_system::Pallet::<Test>::block_number()))
        );

        let _ = PoeModule::remove_claim(Origin::signed(1), claim.clone());

        assert_eq!(
            Proof::<Test>::get(&bounded_claim),
            None
        );
    })
}

#[test]
fn tranfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        let bounded_claim = 
            BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

        assert_eq!(
            Proof::<Test>::get(&bounded_claim),
            Some((1,frame_system::Pallet::<Test>::block_number()))
        );

        let dest = 0;
        let _ = PoeModule::tranfer_claim(Origin::signed(1), claim.clone(),dest);

        let (new,_) = Proof::<Test>::get(&bounded_claim).unwrap();

        assert_eq!(
            new,
            dest
        );

    })
}