#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_module, Parameter,
	traits::{EnsureOrigin, Box},
	dispatch::Dispatchable,
};
use sp_runtime::RuntimeDebug;
use codec::{Encode, Decode};

pub trait Trait: frame_system::Trait {
	type Origin: From<RawOrigin>;
	type Call: Parameter + Dispatchable<Origin=<Self as Trait>::Origin>;
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode)]
pub enum RawOrigin {
	Dumbo,
}

pub type Origin = RawOrigin;

pub struct EnsureDumbo;
impl<
	O: Into<Result<RawOrigin, O>> + From<RawOrigin>,
> EnsureOrigin<O> for EnsureDumbo {
	type Success = ();
	fn try_origin(o: O) -> Result<Self::Success, O> {
		o.into().and_then(|o| match o {
			RawOrigin::Dumbo => Ok(()),
		})
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: <T as frame_system::Trait>::Origin {
		#[weight = 0]
		pub fn become_dumbo(origin, call: Box<<T as Trait>::Call>) {
			let _ = call.dispatch(RawOrigin::Dumbo.into());
		}
	}
}
