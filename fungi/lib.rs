#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_metadata {
    use ink::prelude::vec::Vec;
    use openbrush::traits::String;
    use openbrush::{
        contracts::psp34::extensions::metadata::*, contracts::psp34::extensions::mintable::*,
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP34 for Contract {}

    impl PSP34Metadata for Contract {}
    impl PSP34Mintable for Contract {}

    impl Contract {
        /// A constructor which mints the first token to the owner
        #[ink(constructor)]
        pub fn new(
            id: Id,
            name: String,
            symbol: String,
            ipfshash: String,
            ipfslink: String,
        ) -> Self {
            let mut instance = Self::default();
            let name_key: Vec<u8> = String::from("name");
            let symbol_key: Vec<u8> = String::from("symbol");
            let ipfs_hash: Vec<u8> = String::from("ipfshash");
            let ipfs_link: Vec<u8> = String::from("ipfslink");
            instance._set_attribute(id.clone(), name_key, name);
            instance._set_attribute(id.clone(), symbol_key, symbol);
            instance._set_attribute(id.clone(), ipfs_hash, ipfshash);
            instance._set_attribute(id, ipfs_link, ipfslink);
            instance
        }
    }
}
