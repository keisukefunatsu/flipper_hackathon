#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod flipper {
    use ink::storage::Mapping;
    use openbrush::{contracts::ownable::*, traits::Storage};
    #[ink(event)]
    pub struct Flipped {
        #[ink(topic)]
        caller: Option<AccountId>,
    }

    type FlipperAccounts = Mapping<AccountId, bool>;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Flipper {        
        #[storage_field]
        ownable: ownable::Data,
        flipper_accounts: FlipperAccounts,
    }
    impl Ownable for Flipper {}
    impl Flipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();            
            instance._init_with_owner(Self::env().caller());
            instance.flipper_accounts = Mapping::default();
            // ink::env::debug_println!("created new instance at {}", Self::env().block_number());
            instance
        }

        #[ink(message)]
        pub fn flip(&mut self) {     
            let caller = Self::env().caller();       
            Self::env().emit_event(Flipped {
                caller: Some(caller),                
            });            
            self.flipped();            
        }        
        #[ink(message)]
        pub fn is_flippers(&self, caller: AccountId) -> bool {            
            self.flipper_accounts.get(caller).unwrap_or(false)
        }
        #[ink(message)]
        pub fn flipped(&mut self) {
            let caller = Self::env().caller();
            if self.is_flippers(caller) {
                self.flipper_accounts.remove(caller);
            }
            else {
                self.flipper_accounts.insert(caller, &true);
            }            
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn get_flipped() {
            let accounts = default_accounts();
            let alice: AccountId = accounts.alice.into();
            let bob: AccountId = accounts.bob.into();
            set_sender(alice);
            let mut flipper = Flipper::new();
            flipper.flip(); 
            assert_eq!(flipper.is_flippers(alice), true);
            assert_eq!(flipper.is_flippers(bob), false);
        }        

        // https://github.com/achimcc/contract_get_account_id/blob/470874363e297d45767d9cb3c76200682d459c9c/lib.rs#L175
        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }
    }
}
