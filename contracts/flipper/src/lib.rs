#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod flipper {

    use ink::prelude::string::String;
    use ink::storage::Mapping;

    #[ink(event)]
    pub struct Flipped {
        #[ink(topic)]
        caller: AccountId,
        next_pass: String,
    }
    type FlipperAccounts = Mapping<AccountId, bool>;
    #[ink(storage)]
    #[derive(Default)]
    pub struct Flipper {
        flipper_accounts: FlipperAccounts,
        flip_message: String,
        owner: Option<AccountId>,
    }

    // fn hash_keccak_256(input: &[u8]) -> [u8; 32] {
    //     let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
    //     ink::env::hash_bytes::<hash::Keccak256>(input, &mut output);
    //     output
    // }

    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(message: String) -> Self {
            Self {
                flipper_accounts: Mapping::default(),
                flip_message: message,
                owner: Some(Self::env().caller()),
            }
        }

        #[ink(message)]
        pub fn flip(&mut self, pass: String, alternate_pass: String) {
            if pass.is_empty() || !self.message_check(pass) {
                return;
            }
            let caller = Self::env().caller();
            self.flipped(alternate_pass.clone());
            Self::env().emit_event(Flipped {
                caller,
                next_pass: alternate_pass,
            });
        }

        #[ink(message)]
        pub fn flipped(&mut self, alternate: String) {
            let caller = Self::env().caller();
            // ink::env::debug_println!("{:?}", self.get_flip_message());
            if !self.is_flippers(caller) {
                self.flipper_accounts.insert(caller, &true);
                self.flip_message = alternate
            }
        }

        fn is_flippers(&self, caller: AccountId) -> bool {
            self.flipper_accounts.get(caller).unwrap_or(false)
        }

        fn message_check(&self, pass: String) -> bool {
            // let pass_byte = pass.as_bytes();
            // let digest = hash_keccak_256(pass_byte);
            // TODO: sha256で認証したかった
            if pass == self.flip_message {
                return true;
            }
            false
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
            // TODO: sha256でなんかする
            let mut flipper = Flipper::new(String::from("first_pass"));
            assert_eq!(flipper.is_flippers(alice), false);
            assert_eq!(flipper.is_flippers(bob), false);

            flipper.flip(String::from("first_pass"), String::from("second_pass"));
            assert_eq!(flipper.is_flippers(alice), true);
            assert_eq!(flipper.is_flippers(bob), false);

            set_sender(bob);
            flipper.flip(String::from("second_pass"), String::from("third_pass"));
            assert_eq!(flipper.is_flippers(alice), true);
            assert_eq!(flipper.is_flippers(bob), true);
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
