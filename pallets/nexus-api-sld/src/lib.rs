#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult};
use frame_system::ensure_signed;

use sp_std::prelude::*;

use core::fmt::Debug;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

#[derive(Encode, Decode, Default, Clone, Debug, Eq, PartialEq)]
pub struct Sld {
	iban: bool,
	country_id: Vec<u8>,
	local_bank_number: Vec<u8>,
	local_bank_id: Vec<u8>,
	alias_conversion: bool,
	alias_name: Vec<u8>,
	alias_format: Vec<u8>,
	alias_desc: Vec<u8>,
	max_destination_value: Vec<u8>,
	account_validation_available: bool,
	payee_type: bool,
	ips_timeout: Vec<u8>,
}

decl_storage! {
	trait Store for Module<T: Config> as NexusApiSLD {
		UpdateSld get(fn update_sld): map hasher(blake2_128_concat) (T::AccountId, Vec<u8>) => Sld;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// IPS has input the details
		InputSet(
			AccountId,
			bool,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			bool,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			bool,
			bool,
			Vec<u8>,
		),

		OutputSet(
			AccountId,
			bool,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			bool,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			bool,
			bool,
			Vec<u8>,
		),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		InvalidAccountId,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		// Initialize errors
		type Error = Error<T>;

		// Initialize events
		fn deposit_event() = default;

		#[weight = 10_000_000]
		fn set_info(origin, iban: bool,
			country_id: Vec<u8>,
			local_bank_number: Vec<u8>,
			local_bank_id: Vec<u8>,
			alias_conversion: bool,
			alias_name: Vec<u8>,
			alias_format: Vec<u8>,
			alias_desc: Vec<u8>,
			max_destination_value: Vec<u8>,
			account_validation_available: bool,
			payee_type: bool,
			ips_timeout: Vec<u8>,) -> DispatchResult {
			let user = ensure_signed(origin)?;
			let local_bank_clone = local_bank_number.clone();
			let local_bank_id_clone = local_bank_id.clone();
			let alias_name_clone = alias_name.clone();
			let alias_format_clone = alias_format.clone();
			let alias_desc_clone = alias_desc.clone();
			let user_clone = user.clone();
			let country_id_clone = country_id.clone();
			let ips_timeout_clone = ips_timeout.clone();
			let max_destination_value_clone = max_destination_value.clone();


			let sld = Sld{
				iban,
				country_id,
				local_bank_number,
				local_bank_id,
				alias_conversion,
				alias_name,
				alias_format,
				alias_desc,
				max_destination_value,
				account_validation_available,
				payee_type,
				ips_timeout,
			};

			<UpdateSld<T>>::insert((&user, &country_id_clone), sld);

			Self::deposit_event(RawEvent::InputSet( user_clone, iban,country_id_clone, local_bank_clone, local_bank_id_clone, alias_conversion, alias_name_clone, alias_format_clone, alias_desc_clone, max_destination_value_clone, account_validation_available, payee_type, ips_timeout_clone));
			Ok(())
			}

			#[weight = 10_000]
			fn get_info(origin, country_id : Vec<u8>) -> DispatchResult {
				let getter = ensure_signed(origin)?;

				let keys  = (&getter, country_id);

				let keys_clone = keys.clone();

				let sld = <UpdateSld<T>>::get(keys_clone);
				Self::deposit_event(RawEvent::OutputSet( getter,sld.iban,  sld.country_id , sld.local_bank_number, sld.local_bank_id, sld.alias_conversion, sld.alias_name, sld.alias_format, sld.alias_desc, sld.max_destination_value, sld.account_validation_available, sld.payee_type, sld.ips_timeout));
				Ok(())
			}
	}
}
