use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_stores_correctly(){
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_product(Origin::signed(1),1234,"DELL","Hyderabad"));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::ProductInformation(), Some(1234,"DELL","Hyderabad"));
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::LocationUpdate(Origin::signed(1),1234,"DELL","Bangalore"));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::ProductInformation(), Some(1234,"DELL","Bangalore"));
	});
}

/*
#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}
*/
