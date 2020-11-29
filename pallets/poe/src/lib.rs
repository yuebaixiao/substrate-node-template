#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{
	decl_module, decl_storage, decl_event, 
	ensure, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;

use frame_system::Module;


use sp_std::prelude::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoded(AccountId, Vec<u8>),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProoofAlreadyExist,
		ProoofNotExist,
		NotClaimOwner,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn create_claim(origin, claim::Vec<u8>) => dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&clain), Error::<T>::ProoofAlreadyExist);
			Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Module::<T>::block_number()));
			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

			Ok(())
		}

		#[weight = 10_000]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&clain), Error::<T>::ProoofNotExist);
			
			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRevoded(sender, claim));

			Ok(())
		}
	}
}
