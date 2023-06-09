#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod my_psp22 {
    
    // imports from openbrush
	use openbrush::traits::String;
	use openbrush::traits::Storage;
	use openbrush::contracts::psp22::extensions::mintable::*;
	use openbrush::contracts::psp22::extensions::metadata::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Fungi {
    	#[storage_field]
		psp22: psp22::Data,
		#[storage_field]
		metadata: metadata::Data,
    }
    
    // Section contains default implementation without any modifications
	impl PSP22 for Fungi {}
	impl PSP22Mintable for Fungi {}
	impl PSP22Metadata for Fungi {}
     
    impl Fungi {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut _instance = Self::default();
			_instance._mint_to(_instance.env().caller(), initial_supply).expect("Should mint"); 
			_instance.metadata.name = name;
			_instance.metadata.symbol = symbol;
			_instance.metadata.decimals = decimal;
			_instance
        }
    }
}
