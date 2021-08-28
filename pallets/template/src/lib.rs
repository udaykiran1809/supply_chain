#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec; 

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // Pallets use events to inform users when important changes are made.
    // Event documentation should end with an array that provides descriptive names for parameters.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a product is added.
        ProductAdded(T::AccountId, Vec<u8>),
    	/// Event emitted when location is updated.
        LocationUpdated(Vec<u8>),
    }
    
    #[pallet::error]
    pub enum Error<T> {
            /// Entered Product already Exists.
            ProductAlreadyExists,
            /// Entered Product doesn't exist.
            NoSuchProduct,
	    /// Productid doesn't match with the productname.
	    NoMatchFound,
        }
    
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    
	#[pallet::storage] 
	pub(super) type ProductInformation<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (Vec<u8>,Vec<u8>,T::AccountId), ValueQuery>;	

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    
    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub(super) fn add_product(
			origin: OriginFor<T>,
			productid: Vec<u8>,
			productname: Vec<u8>,
			location: Vec<u8>,
		) -> DispatchResultWithPostInfo {

			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;
		
			// To check whether given product exists or not.         
			ensure!(!ProductInformation::<T>::contains_key(&productid), Error::<T>::ProductAlreadyExists);

			// Store product value.
			ProductInformation::<T>::insert(&productid, (&location,&productname,&sender));

			// Emit an event that store a product information.
			Self::deposit_event(Event::ProductAdded(sender,productid));

			Ok(().into())
		}
		#[pallet::weight(10_000)]
		fn LocationUpdate(
			origin: OriginFor<T>,
			productid: Vec<u8>,
			productname: Vec<u8>,
			location: Vec<u8>,
		 ) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			
			let sender = ensure_signed(origin)?;

			// Verify product exists or not.
			ensure!(ProductInformation::<T>::contains_key(&productid), Error::<T>::NoSuchProduct);
			ensure!(ProductInformation::<T>::contains_key(&productname), Error::<T>::NoMatchFound);
			
			ProductInformation::<T>::remove(&location);

			ProductInformation::<T>::insert(&productid, (&location,&productname,&sender));

			// Emit an event that location is updated.
			Self::deposit_event(Event::LocationUpdated(location));

			Ok(().into())
		}
	}
}
