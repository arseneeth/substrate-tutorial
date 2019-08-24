use support::{decl_storage, decl_module, StorageMap, StorageValue, ensure};
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
    trait Store for Module<T: Trait> as KittyStorage {
    
        Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
		KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;
        OwnedKitty get(kitty_of_owner): map T::AccountId => T::Hash;

        Nonce: u64;

    }
}


decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn create_kitty(origin) -> Result {

            let sender = ensure_signed(origin)?;

            let nonce = <Nonce<T>>::get();
            let random_seed = <system::Module<T>>::random_seed();
            let random_hash = (random_seed, &sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);

			ensure!(!<Kitties<T>>::exists(random_hash), "This new id already exists");

			// let hash_of_zero = <T as system::Trait>::Hashing::hash_of(&0);
			// let my_zero_balance = <T::Balance as As<u64>>::sa(0);

            let new_kitty = Kitty {
            	id: random_hash,
       			dna: random_hash,
       			price: <T::Balance as As<u64>>::sa(0),     	
            	gen: 0,
            };

            <Kitties<T>>::insert(random_hash, new_kitty);

            <KittyOwner<T>>::insert(random_hash, &sender);

            <OwnedKitty<T>>::insert(&sender, random_hash);

			<Nonce<T>>::mutate(|n| *n += 1);

            Ok(())
        }


    }
}
