#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::*, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub enum Zodiac {
		Aquarius,
		Pisces,
		Aries,
		Taurus,
		Gemini,
		Cancer,
		Leo,
		Virgo,
		Libra,
		Scorpio,
		Sagittarius,
		Capricorn,
	}

	impl Default for Zodiac {
		fn default() -> Self { Zodiac::Aquarius }
	}


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn account_zodiac)]
	//Account's zodiac storage
	pub type AccountZodiac<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Zodiac, ValueQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetZodiacSucceed(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ZodiacWasSet,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000)]
		pub fn setZodiac(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			if AccountZodiac::<T>::contains_key(&who) {
				Err(Error::<T>::ZodiacWasSet)?
			} else {
				AccountZodiac::<T>::insert(&who, Zodiac::Aquarius);
				Self::deposit_event(Event::<T>::SetZodiacSucceed(who));
			}
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}