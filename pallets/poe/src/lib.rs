#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, BoundedVec};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
        type MaxClaimLength: Get<u32>;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Proof<T:Config> = StorageMap<
    _, 
    Blake2_128Concat,
    BoundedVec<u8, T::MaxClaimLength>,
    (T::AccountId, T::BlockNumber),
    >;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRemoved(T::AccountId, Vec<u8>),
		ClaimTranfered(T::AccountId, Vec<u8>),
	}


	#[pallet::error]
	pub enum Error<T> {
		ClaimTooLong,
		ProofAlreadyExit,
		NoneValue,
		NotClaimOwner,


	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let bounded_claim =BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
			.map_err(|_| Error::<T>::ClaimTooLong)?;
			ensure!(!Proof::<T>::contains_key(&bounded_claim), Error::<T>::ProofAlreadyExit);

			/// block_number可以直接得出
			<Proof<T>>::insert(
				&bounded_claim,
				(sender.clone(),frame_system::Pallet::<T>::block_number()),
			);

			Self::deposit_event(Event::ClaimCreated(sender, claim));
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(0)]
		pub fn remove_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let bounded_claim =BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
			.map_err(|_| Error::<T>::ClaimTooLong)?;

			let (owner,_) = Proof::<T>::get(&bounded_claim).unwrap();
			ensure!(owner==sender, Error::<T>::NotClaimOwner);

			Proof::<T>::remove(&bounded_claim);
			Self::deposit_event(Event::ClaimRemoved(sender, claim));
			
			Ok(())						
			}

		#[pallet::weight(0)]
		pub fn tranfer_claim(origin: OriginFor<T>, claim: Vec<u8>, dest: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let bounded_claim =BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
			.map_err(|_| Error::<T>::ClaimTooLong)?;

			let (owner, _blockNumber) = Proof::<T>::get(&bounded_claim).unwrap();
			ensure!(owner==sender, Error::<T>::NotClaimOwner);

			Proof::<T>::insert(&bounded_claim,(dest,frame_system::Pallet::<T>::block_number()));
			Self::deposit_event(Event::ClaimTranfered(sender, claim));
			
			Ok(())						
			}
		}
}
