use codec::{Decode, Encode};
#[cfg(feature = "std")]
use parity_util_mem::{MallocSizeOf, MallocSizeOfOps};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{RuntimeDebug, H256};

use crate::{asdr::DataLookup, KateCommitment};

#[derive(PartialEq, Eq, Clone, RuntimeDebug, TypeInfo, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct HeaderExtension {
	pub app_lookup: DataLookup,
	pub commitment: KateCommitment,
}

impl HeaderExtension {
	pub fn data_root(&self) -> H256 { self.commitment.data_root }
}

#[cfg(feature = "std")]
impl MallocSizeOf for HeaderExtension {
	fn size_of(&self, ops: &mut MallocSizeOfOps) -> usize {
		self.commitment.size_of(ops) + self.app_lookup.size_of(ops)
	}
}
