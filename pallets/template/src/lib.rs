#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_event};
use frame_support::traits::EnsureOrigin;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
	type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;
	type DumboOrigin: EnsureOrigin<Self::Origin>;
}

decl_event!(
	pub enum Event {
		Success,
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: <T as frame_system::Trait>::Origin {
		fn deposit_event() = default;

		#[weight = 0]
		fn only_dumbo(origin) {
			T::DumboOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::Success);
		}
	}
}
