#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod flipper {
    use ink::{prelude::string::String, storage::Mapping};
    use openbrush::{contracts::ownable::*, traits::Storage};
    #[ink(event)]
    pub struct Flipped {
        #[ink(topic)]
        from: Option<AccountId>,

        #[ink(topic)]
        message: Option<String>,
    }

    type FlipperAccounts = Mapping<AccountId, bool>;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Flipper {
        value: bool,
        #[storage_field]
        ownable: ownable::Data,
        flipper_accounts: FlipperAccounts,
    }
    impl Ownable for Flipper {}
    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let mut instance = Self::default();
            instance.value = init_value;
            instance._init_with_owner(Self::env().caller());
            instance.flipper_accounts = Mapping::default();
            // ink::env::debug_println!("created new instance at {}", Self::env().block_number());
            instance
        }

        #[ink(message)]
        pub fn flip(&mut self) -> bool {
            self.value = !self.value;
            Self::env().emit_event(Flipped {
                from: Some(Self::env().caller()),
                message: Some(String::from("string")),
            });
            self.value
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
        #[ink(message)]
        pub fn is_flippers(&self) -> bool {
            let caller = Self::env().caller();
            self.flipper_accounts.get(caller).unwrap_or(false)
        }
        #[ink(message)]
        pub fn set_flippers(&mut self) {
            let caller = Self::env().caller();
            self.flipper_accounts.insert(caller, &true);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn get_flipped() {
            let accounts = default_accounts();
            let alice: AccountId = accounts.alice.into();
            set_sender(alice);
            let mut flipper = Flipper::new(false);
            flipper.flip();
            flipper.set_flippers();
            assert_eq!(flipper.is_flippers(), true);
        }
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
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
