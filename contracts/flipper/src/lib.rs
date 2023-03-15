#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod flipper {
    use openbrush::{contracts::ownable::*, traits::Storage};

    #[ink(storage)]
    #[derive(Storage)]
    pub struct Flipper {
        #[storage_field]
        ownable: ownable::Data,
        value: bool,
    }
    impl Ownable for Flipper {}
    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let mut instance = Self::default();
            instance.value = init_value;
            instance._init_with_owner(Self::env().caller());
            // ink::env::debug_println!("created new instance at {}", Self::env().block_number());
            instance
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                value: false,
                ownable: ownable::Data::default(),
            }
        }

        #[ink(message)]
        pub fn flip(&mut self) -> bool {
            self.value = !self.value;
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
