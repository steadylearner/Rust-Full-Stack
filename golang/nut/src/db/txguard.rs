use crate::db::DB;
use crate::tx::Tx;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Guard returned by DB.begin_tx()
///
/// Statically guards against outliving db
/// and prevents from making mutable actions.
///
/// Implements Deref to Tx
pub struct TxGuard<'a> {
	pub(crate) tx: Tx,
	pub(crate) db: PhantomData<&'a DB>,
}

impl<'a> Deref for TxGuard<'a> {
	type Target = Tx;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.tx
	}
}

/// Guard returned by DB.begin_rw_tx()
///
/// Statically guards against multiple mutable db borrows.
///
/// Implements Deref and DerefMut to Tx
pub struct RWTxGuard<'a> {
	pub(crate) tx: Tx,
	pub(crate) db: PhantomData<&'a mut DB>,
}

impl<'a> Deref for RWTxGuard<'a> {
	type Target = Tx;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.tx
	}
}

impl<'a> DerefMut for RWTxGuard<'a> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.tx
	}
}
