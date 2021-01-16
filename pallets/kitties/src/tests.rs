use crate::{Event, Error, mock::*, Trait};
use frame_support::{assert_noop, assert_ok, traits::{OnFinalize, OnInitialize}};

fn run_to_block( n: u64) {
	while System::block_number() < n {
		KittiesModule::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number()+1);
		System::on_initialize(System::block_number());
		KittiesModule::on_initialize(System::block_number());
	}
}

// 测试创建一个 Kitty
#[test]
fn create_kitty_works(){
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_ok!(KittiesModule::create( Origin::signed(1)) );

		assert_eq!(
			System::events()[0].event,
			TestEvent::kitties( Event::<Test>::Created( 1u64 , 0) )
		);
	})
}

// 测试创建一个 Kitty
#[test]
fn transfer_kitty_works(){
	new_test_ext().execute_with(|| {
		run_to_block(10);
		let _ = KittiesModule::create( Origin::signed(2) );

		assert_ok!(KittiesModule::transfer( Origin::signed(20), 2, 0 ) );

		assert_eq!(
			System::events()[1].event,
			TestEvent::kitties( Event::<Test>::Transferred( 20u64 ,2u64, 0) )
		);
	})
}

#[test]
fn transfer_kitty_failed_when_not_exists(){
	new_test_ext().execute_with(|| {
		assert_noop!(KittiesModule::transfer( Origin::signed(1), 2, 0 ) , Error::<Test>::KittyNotExists);
	})
}