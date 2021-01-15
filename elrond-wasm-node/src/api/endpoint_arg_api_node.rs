use crate::ext_error;
use crate::ArwenApiImpl;
use alloc::vec::Vec;
use elrond_wasm::api::ContractHookApi;
use elrond_wasm::api::EndpointArgumentApi;
use elrond_wasm::err_msg;
use elrond_wasm::types::BoxedBytes;

#[rustfmt::skip]
extern {
	fn getNumArguments() -> i32;
    fn getArgumentLength(id: i32) -> i32;
	fn getArgument(id: i32, dstOffset: *mut u8) -> i32;

	// fn callValue(resultOffset: *const u8) -> i32;
    // fn getESDTValue(resultOffset: *const u8) -> usize;
    // fn getESDTTokenName(resultOffset: *const u8) -> usize;
	
	// big int API
    fn bigIntNew(value: i64) -> i32;
    fn bigIntGetUnsignedArgument(arg_id: i32, dest: i32);
    fn bigIntGetSignedArgument(arg_id: i32, dest: i32);
    // fn bigIntGetCallValue(dest: i32);
    // fn bigIntGetESDTCallValue(dest: i32);
	
	// small int API
    fn smallIntGetUnsignedArgument(id: i32) -> i64;
    fn smallIntGetSignedArgument(id: i32) -> i64;
}

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
impl EndpointArgumentApi for ArwenApiImpl {
	#[inline]
	fn get_num_arguments(&self) -> i32 {
		unsafe { getNumArguments() }
	}

	fn check_not_payable(&self) {
		if self.get_call_value_big_uint() > 0 {
			ext_error::signal_error(err_msg::NON_PAYABLE);
		}
	}

	#[inline]
	fn get_argument_len(&self, arg_index: i32) -> usize {
		unsafe { getArgumentLength(arg_index) as usize }
	}

	fn copy_argument_to_slice(&self, arg_index: i32, slice: &mut [u8]) {
		unsafe {
			let byte_len = getArgument(arg_index, slice.as_mut_ptr()) as usize;
			if byte_len != slice.len() {
				ext_error::signal_error(err_msg::ARG_BAD_LENGTH);
			}
		}
	}

	fn get_argument_vec_u8(&self, arg_index: i32) -> Vec<u8> {
		let len = self.get_argument_len(arg_index);
		let mut res = Vec::with_capacity(len);
		if len > 0 {
			unsafe {
				res.set_len(len);
				getArgument(arg_index, res.as_mut_ptr());
			}
		}
		res
	}

	fn get_argument_boxed_bytes(&self, arg_index: i32) -> BoxedBytes {
		let len = self.get_argument_len(arg_index);
		unsafe {
			let mut res = BoxedBytes::allocate(len);
			if len > 0 {
				getArgument(arg_index, res.as_mut_ptr());
			}
			res
		}
	}

	fn get_argument_big_uint_raw(&self, arg_id: i32) -> i32 {
		unsafe {
			let handle = bigIntNew(0);
			bigIntGetUnsignedArgument(arg_id, handle);
			handle
		}
	}

	fn get_argument_big_int_raw(&self, arg_id: i32) -> i32 {
		unsafe {
			let handle = bigIntNew(0);
			bigIntGetSignedArgument(arg_id, handle);
			handle
		}
	}

	#[inline]
	fn get_argument_u64(&self, arg_id: i32) -> u64 {
		unsafe { smallIntGetUnsignedArgument(arg_id) as u64 }
	}

	#[inline]
	fn get_argument_i64(&self, arg_id: i32) -> i64 {
		unsafe { smallIntGetSignedArgument(arg_id) }
	}
}
