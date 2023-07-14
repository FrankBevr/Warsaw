#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod my_psp37 {
    
    // imports from openbrush
	use openbrush::traits::String;
	use openbrush::traits::Storage;
	use openbrush::contracts::psp37::extensions::mintable::*;
	use openbrush::contracts::psp37::extensions::metadata::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Housi {
    	#[storage_field]
		psp37: psp37::Data,
		#[storage_field]
		metadata: metadata::Data,
    }
    
    // Section contains default implementation without any modifications
	impl PSP37 for Housi {}
	impl PSP37Mintable for Housi {}
	impl PSP37Metadata for Housi {}
     
    impl Housi {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			_instance
        }
    }
}
