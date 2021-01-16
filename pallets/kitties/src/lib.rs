#![cfg_attr(not(feature = "std"), no_std)] //must need

use codec::{Encode, Decode};
use frame_support::{decl_module,decl_storage, decl_event, decl_error, StorageValue, ensure, StorageMap, traits::Randomness, Parameter,traits::{Get, Currency, ReservableCurrency}
};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;
use sp_runtime::{DispatchError,traits::{AtLeast32Bit,Bounded}};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//type KittyIndex = u32;
// 定义一个 kitty 的数据结构
#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

// 定义 Trait，
pub trait Trait: frame_system::Trait {
    // 如果有触发事件，就必须包含这一行
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Randomness: Randomness<Self::Hash>;

	// 定义 KittyIndex 类型，要求实现指定的 trait 
	// Parameter 表示可以用于函数参数传递
	// AtLeast32Bit 表示转换为 u32 不会造成数据丢失
	// Bounded 表示包含上界和下界
	// Default 表示有默认值
	// Copy 表示可以实现 Copy 方法
	type KittyIndex: Parameter + AtLeast32Bit + Bounded + Default + Copy;
}

// 定义事件
decl_event!(
	// where 后边的部分，是表示在 Event 里边需要用的一些类型来自哪个 Trait 定义
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId, KittyIndex = <T as Trait>::KittyIndex {
		Created(AccountId, KittyIndex),
		Transferred(AccountId, AccountId, KittyIndex),
	}
);


// 定义错误信息
decl_error! {
	pub enum Error for Module<T: Trait> {
        KittiesCountOverflow,
        InvalidKittyId,
		KittyNotExists,
		NotKittyOwner,
		TransferToSelf,
		RequiredDiffrentParent,
		MoneyNotEnough,
		UnReserveMoneyNotEnough,
	}
}

// 定义数据存储
decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		// 保存所有 kitty 的数据，用 KittyIndex 作为健值
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;
        // 保存 kitty 的总数，严格上来说，应该是最大的 Kitty 的健值索引，因为如果支持 kitty 的删除，实现上就不对了。
        pub KittiesCount get(fn kitties_count): T::KittyIndex;
        // 保存每一只猫归那个拥有者
        pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) T::KittyIndex => Option<T::AccountId>;
        
    }
}

// 定义可被调用的方法
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		// 如果有触发错误信息，必须包含这一行
		type Error = Error<T>;
		// 如果有触发事件，必须包含这一行
		fn deposit_event() = default;
		
        #[weight = 0]
		pub fn create(origin){
            let sender = ensure_signed(origin)?;
            let kitty_id = Self::next_kitty_id()?;
            let dna = Self::random_value(&sender);
            let kitty = Kitty(dna);

            Self::insert_kitty(&sender, kitty_id, kitty);
            Self::deposit_event(RawEvent::Created(sender, kitty_id));
        }

        #[weight = 0]
		pub fn transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex){
			let sender = ensure_signed(origin)?;
			
			// 判断kitty_id是否存在，如果不存在就报错
			ensure!(<Kitties<T>>::contains_key(&kitty_id), <Error<T>>::KittyNotExists);

			// 从map里得到这只猫的owner
			let owner = <KittyOwners<T>>::get(&kitty_id).unwrap();
			// 如果sender不是kitty_id这只猫的主人，就报NotKittyOwner
			ensure!(owner == sender, Error::<T>::NotKittyOwner);

			// 不可以自己转给自己
			ensure!(to != sender, <Error<T>>::TransferToSelf);

			// 修改kitty_id这只猫的主人，删除原来的主人，添加新的主人
			<KittyOwners<T>>::remove(kitty_id);
            <KittyOwners<T>>::insert(kitty_id, to.clone());
            // 触发转让的事件
			Self::deposit_event(RawEvent::Transferred(sender, to, kitty_id));
        }

        #[weight = 0]
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex){
			let sender = ensure_signed(origin)?;
			let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;

			Self::deposit_event(RawEvent::Created(sender, new_kitty_id));
		}
    }
}

impl<T: Trait> Module<T> {
	// 获取下一个
	fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError>{
		let kitty_id = Self::kitties_count();
		if kitty_id == T::KittyIndex::max_value() {
			return Err(Error::<T>::KittiesCountOverflow.into());
		}
		Ok(kitty_id)
    }

    fn random_value(sender : &T::AccountId) -> [u8; 16] {
		let payload = (
			T::Randomness::random_seed(),	// 通过最近区块信息生成的随机数种子
			&sender,
			<frame_system::Module<T>>::extrinsic_index() // 当前交易在区块中的顺序
		);
		payload.using_encoded(blake2_128)
    }
    
    // 插入一个 kitty ，因为父母可能不存在，所以parent 需要用 Option
	fn insert_kitty(owner : &T::AccountId, kitty_id : T::KittyIndex, kitty : Kitty){
		// 保存 Kitty 
		<Kitties::<T>>::insert(kitty_id, kitty);
		// 更新 Kitty 数量，当前 ID+1
		<KittiesCount::<T>>::put(kitty_id + 1.into());
		// 保存 Kitty 的所有关系
		<KittyOwners<T>>::insert(kitty_id, owner);
    }

    fn do_breed(sender : &T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> sp_std::result::Result<T::KittyIndex, DispatchError>{

		// 判断 KittyIndex 是否存在，通过 ok_or 将错误抛出来，如果没有将返回一个 option 类型的数据
		let kitty1 = Self::kitties(kitty_id_1).ok_or( Error::<T>::InvalidKittyId )?;
        let kitty2 = Self::kitties(kitty_id_2).ok_or( Error::<T>::InvalidKittyId )?;
        
        // 不允许相同的猫进行繁殖
		ensure!( kitty_id_1 != kitty_id_2, Error::<T>::RequiredDiffrentParent);
	
		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.0;
		let kitty2_dna = kitty2.0;

		let selector = Self::random_value(&sender);

		let mut new_dna = [0u8; 16];

		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

		Ok(kitty_id)
	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8{
	(selector & dna1 ) | (!selector & dna2)
}