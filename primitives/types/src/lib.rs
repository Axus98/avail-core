#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use alloc::vec::Vec;
use derive_more::{Add, Constructor, Deref, Display, From, Into, Mul};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;

mod data_lookup;
mod get_app_id;
pub use data_lookup::{DataLookup, DataLookupIndexItem, Error as DataLookupError};
pub use get_app_id::*;

/// Raw Extrinsic with application id.
#[derive(Clone, TypeInfo, Default, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AppExtrinsic {
	pub app_id: AppId,
	pub data: Vec<u8>,
}
#[cfg(feature = "substrate")]
impl<A, C, S, E> From<sp_runtime::generic::UncheckedExtrinsic<A, C, S, E>> for AppExtrinsic
where
	A: Encode,
	C: Encode,
	S: Encode,
	E: sp_runtime::traits::SignedExtension + crate::GetAppId,
{
	fn from(ue: sp_runtime::generic::UncheckedExtrinsic<A, C, S, E>) -> Self {
		let app_id = ue
			.signature
			.as_ref()
			.map(|(_, _, extra)| extra.app_id())
			.unwrap_or_default();
		let data = ue.encode();

		Self { app_id, data }
	}
}

impl GetAppId for AppExtrinsic {
	fn app_id(&self) -> AppId {
		self.app_id
	}
}

#[derive(
	Clone,
	Copy,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Add,
	From,
	Deref,
	TypeInfo,
	Encode,
	Decode,
	Display,
	Into,
	Default,
	MaxEncodedLen,
	RuntimeDebug,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AppId(#[codec(compact)] pub u32);

impl num_traits::Zero for AppId {
	fn zero() -> Self {
		AppId(num_traits::Zero::zero())
	}

	fn is_zero(&self) -> bool {
		self.0.is_zero()
	}
}

impl From<alloc::vec::Vec<u8>> for AppExtrinsic {
	#[inline]
	fn from(data: Vec<u8>) -> Self {
		Self {
			data,
			app_id: <_>::default(),
		}
	}
}

/// Strong type for `BlockLength::cols`
#[derive(
	Clone,
	Copy,
	Debug,
	From,
	Into,
	Add,
	Mul,
	Display,
	PartialEq,
	Eq,
	Encode,
	Decode,
	TypeInfo,
	PartialOrd,
	Ord,
	Constructor,
	MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[mul(forward)]
pub struct BlockLengthColumns(#[codec(compact)] pub u32);

impl BlockLengthColumns {
	#[inline]
	pub fn as_usize(&self) -> usize {
		self.0 as usize
	}
}

/// Strong type for `BlockLength::rows`
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
	Clone,
	Copy,
	Debug,
	From,
	Into,
	Add,
	Mul,
	Display,
	PartialEq,
	Eq,
	Encode,
	Decode,
	TypeInfo,
	PartialOrd,
	Ord,
	Constructor,
	MaxEncodedLen,
)]
#[mul(forward)]
pub struct BlockLengthRows(#[codec(compact)] pub u32);

impl BlockLengthRows {
	#[inline]
	pub fn as_usize(&self) -> usize {
		self.0 as usize
	}
}

/// Return Err of the expression: `return Err($expression);`.
///
/// Used as `fail!(expression)`.
#[macro_export]
macro_rules! fail {
	( $y:expr ) => {{
		return Err($y.into());
	}};
}

/// Evaluate `$x:expr` and if not true return `Err($y:expr)`.
///
/// Used as `ensure!(expression_to_ensure, expression_to_return_on_false)`.
#[macro_export]
macro_rules! ensure {
	( $x:expr, $y:expr $(,)? ) => {{
		if !$x {
			$crate::fail!($y);
		}
	}};
}