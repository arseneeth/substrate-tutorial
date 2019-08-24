use support::{decl_storage, decl_module, StorageMap};
use support::dispatch::Result;
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use parity_codec::{Encode, Decode};

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]

pub struct Kitty<Hash, Balance> {

			   id: Hash,   	   // `id` as a `Hash`
    		   dna: Hash,	   // `dna` as a `Hash`
    		   price: Balance, // `price` as a `Balance` 
    		   gen: u64        // `gen` as a `u64`

}

pub trait Trait: balances::Trait {}

decl_storage! {
    trait OwnedKitty for Module<T: Trait> as KittyStorage {
       
       KittyOfOwner: map T::AccountId => Kitty<T::Hash, T::Balance>

    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn create_kitty(origin) -> Result {

            let sender = ensure_signed(origin)?;
			let hash_of_zero = <T as system::Trait>::Hashing::hash_of(&0);
			let my_zero_balance = <T::Balance as As<u64>>::sa(0);

            let new_kitty = Kitty {
            	id: hash_of_zero,
       			dna: hash_of_zero,
       			price: my_zero_balance,     	
            	gen: 0,
            };

            <KittyOfOwner<T>>::insert(sender, new_kitty);

            Ok(())
        }


    }
}
