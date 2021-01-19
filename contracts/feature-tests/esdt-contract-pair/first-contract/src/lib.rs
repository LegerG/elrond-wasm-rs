#![no_std]
#![allow(unused_attributes)]

imports!();

use elrond_wasm::HexCallDataSerializer;

const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";
const SECOND_CONTRACT_ACCEPT_ESDT_PAYMENT: &[u8] = b"acceptEsdtPayment";
const SECOND_CONTRACT_REJECT_ESDT_PAYMENT: &[u8] = b"rejectEsdtPayment";

#[elrond_wasm_derive::contract(FirstContractImpl)]
pub trait FirstContract {
	#[init]
	fn init(&self, esdt_token_name: BoxedBytes, second_contract_address: Address) {
		self.set_contract_esdt_token_name(&esdt_token_name);
		self.set_second_contract_address(&second_contract_address);
	}

	#[endpoint(transferToSecondContractFull)]
	fn transfer_to_second_contract_full(&self) -> SCResult<()> {
		let expected_token_name = self.get_contract_esdt_token_name();
		let actual_token_name = self.get_esdt_token_name_boxed();
		let esdt_value = self.get_esdt_value_big_uint();

		require!(esdt_value > 0, "no esdt transfered!");
		require!(actual_token_name == expected_token_name, "Wrong esdt token");

		self.call_esdt_second_contract(
			&expected_token_name,
			&esdt_value,
			&self.get_second_contract_address(),
			SECOND_CONTRACT_ACCEPT_ESDT_PAYMENT,
			&[],
		);

		Ok(())
	}

	#[endpoint(transferToSecondContractHalf)]
	fn transfer_to_second_contract_half(&self) -> SCResult<()> {
		let expected_token_name = self.get_contract_esdt_token_name();
		let actual_token_name = self.get_esdt_token_name_boxed();
		let esdt_value = self.get_esdt_value_big_uint();

		require!(esdt_value > 0, "no esdt transfered!");
		require!(actual_token_name == expected_token_name, "Wrong esdt token");

		self.call_esdt_second_contract(
			&expected_token_name,
			&(esdt_value / BigUint::from(2u32)),
			&self.get_second_contract_address(),
			SECOND_CONTRACT_ACCEPT_ESDT_PAYMENT,
			&[],
		);

		Ok(())
	}

	#[endpoint]
	fn transfer_to_second_contract_rejected(&self) -> SCResult<()> {
		let expected_token_name = self.get_contract_esdt_token_name();
		let actual_token_name = self.get_esdt_token_name_boxed();
		let esdt_value = self.get_esdt_value_big_uint();

		require!(esdt_value > 0, "no esdt transfered!");
		require!(actual_token_name == expected_token_name, "Wrong esdt token");

		self.call_esdt_second_contract(
			&expected_token_name,
			&(esdt_value / BigUint::from(2u32)),
			&self.get_second_contract_address(),
			SECOND_CONTRACT_REJECT_ESDT_PAYMENT,
			&[],
		);

		Ok(())
	}

	fn get_esdt_token_name_boxed(&self) -> BoxedBytes {
		BoxedBytes::from(self.get_esdt_token_name())
	}

	fn call_esdt_second_contract(
		&self,
		esdt_token_name: &BoxedBytes,
		amount: &BigUint,
		to: &Address,
		func_name: &[u8],
		args: &[BoxedBytes],
	) {
		let mut serializer = HexCallDataSerializer::new(ESDT_TRANSFER_STRING);
		serializer.push_argument_bytes(esdt_token_name.as_slice());
		serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
		serializer.push_argument_bytes(func_name);
		for arg in args {
			serializer.push_argument_bytes(arg.as_slice());
		}

		self.async_call(&to, &BigUint::zero(), serializer.as_slice());
	}

	// storage

	#[storage_set("esdtTokenName")]
	fn set_contract_esdt_token_name(&self, esdt_token_name: &BoxedBytes);

	#[view(getEsdtTokenName)]
	#[storage_get("esdtTokenName")]
	fn get_contract_esdt_token_name(&self) -> BoxedBytes;

	#[storage_set("secondContractAddress")]
	fn set_second_contract_address(&self, address: &Address);

	#[view(getSecondContractAddress)]
	#[storage_get("secondContractAddress")]
	fn get_second_contract_address(&self) -> Address;
}