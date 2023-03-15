#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod flipper {
    use ink::prelude::string::String;
    use openbrush::{contracts::ownable::*, traits::Storage};
    #[ink(event)]
    pub struct Flipped {
        #[ink(topic)]
        from: Option<AccountId>,

        #[ink(topic)]
        message: Option<String>,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Flipper {
        #[storage_field]
        ownable: ownable::Data,
        value: bool,
    }
    impl Ownable for Flipper {}
    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let mut instance = Self::default();
            instance.value = init_value;
            instance._init_with_owner(Self::env().caller());
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
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
    }
}
