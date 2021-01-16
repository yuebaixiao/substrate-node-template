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

// 测试转移一个 Kitty
#[test]
fn transfer_kitty_works(){
	new_test_ext().execute_with(|| {
		run_to_block(10);

		// 生产第一个猫，所以是第0号，所以下面调用transfer方法的第3个参数必须设置成0，
		// 否则就出KittyNotExists错误了。
		let _ = KittiesModule::create( Origin::signed(1) );

		assert_ok!(KittiesModule::transfer( Origin::signed(1), 2, 0) );

		// assert_eq!(
		// 	System::events()[1].event,
		// 	TestEvent::kitties( Event::<Test>::Transferred( 20u64 ,2u64, 0) )
		// );
	})
}

// 测试转移一个不存在的 Kitty
#[test]
fn transfer_kitty_failed_when_not_exists() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_noop!(KittiesModule::transfer( Origin::signed(1), 1, 0),
			Error::<Test>::KittyNotExists );
	})
}

// 测试转移一个不是owner的 Kitty
#[test]
fn transfer_kitty_failed_when_not_owner() {
	new_test_ext().execute_with(|| {
		run_to_block(10);

		let _ = KittiesModule::create( Origin::signed(1) );

		// 上面是用signed(1)创建的猫，转移的时候，使用signed(2)，所以不是0号猫的owner
		assert_noop!(KittiesModule::transfer( Origin::signed(2), 1, 0),
			Error::<Test>::NotKittyOwner);

	})
}

// 测试转移给自己 Kitty
#[test]
fn transfer_kitty_failed_when_to_self() {
	new_test_ext().execute_with(|| {
		run_to_block(10);

		let _ = KittiesModule::create( Origin::signed(1) );

		// 由于第二参数是新的主人，但是id是1和原来的一样了，所以error
		assert_noop!(KittiesModule::transfer( Origin::signed(1), 1, 0),
			Error::<Test>::TransferToSelf);

	})
}

// 测试繁殖一个 Kitty
#[test]
fn breed_kitty_works(){
	new_test_ext().execute_with(|| {
		run_to_block(10);
		let _ = KittiesModule::create( Origin::signed(1) );
		let _ = KittiesModule::create( Origin::signed(2) );

		assert_ok!(KittiesModule::breed( Origin::signed(1), 0, 1) );
	})
}