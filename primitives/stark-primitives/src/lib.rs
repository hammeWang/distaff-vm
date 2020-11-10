#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Encode, Decode};
use sp_runtime_interface::{runtime_interface, pass_by::{PassBy, PassByInner}};

#[cfg(feature = "std")]
use distaff::StarkProof;

///// For testing purpose
//#[runtime_interface]
//pub trait Print {
//	fn print() {
//		use std::vec::Vec;
//		let a = vec![1, 2, 3];
//		println!("Hello world!, {:?}", a);
//	}
//}

#[runtime_interface]
pub trait Stark {
	fn st_verify(
		program_hash: &[u8; 32],
		public_inputs: &[u128],
		outputs: &[u128],
		proof: &[u8]
	) {
		let stark_proof = bincode::deserialize::<StarkProof>(&proof).unwrap();
		match distaff::verify(program_hash, public_inputs, outputs, &stark_proof) {
			Ok(r) => Some(r),
			Err(_) => None
		};
	}
}