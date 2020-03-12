use std::fmt;

use super::branch_page_element::BranchPageElement;
use super::leaf_page_element::LeafPageElement;
use crate::consts::{Flags, PGID};
use crate::meta::Meta;
use crate::utils::{slice_to_u8_slice, to_u8_slice};

pub(crate) enum PageData<'a> {
	Branches(&'a [BranchPageElement]),
	Leaves(&'a [LeafPageElement]),
	Meta(&'a Meta),
	Freelist(&'a [PGID]),
}

impl<'a> fmt::Display for PageData<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let d = match self {
			PageData::Branches(_) => "branches".to_string(),
			PageData::Leaves(_) => "leaves".to_string(),
			PageData::Meta(_) => "meta".to_string(),
			PageData::Freelist(_) => "freelist".to_string(),
		};
		write!(f, "{}", d)
	}
}

impl<'a> PageData<'a> {
	#[allow(dead_code)]
	pub(crate) fn typ(&self) -> Flags {
		match self {
			PageData::Branches(_) => Flags::BRANCHES,
			PageData::Leaves(_) => Flags::LEAVES,
			PageData::Meta(_) => Flags::META,
			PageData::Freelist(_) => Flags::FREELIST,
		}
	}

	// TODO: hmmmm
	pub(crate) fn as_bytes(&self) -> &[u8] {
		match self {
			PageData::Branches(d) => unsafe { slice_to_u8_slice(d) },
			PageData::Leaves(d) => unsafe { slice_to_u8_slice(d) },
			PageData::Meta(d) => unsafe { to_u8_slice(*d) },
			PageData::Freelist(d) => unsafe { slice_to_u8_slice(d) },
		}
	}
}
