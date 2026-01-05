use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{impl_memset, DismOnlyIxData};

// Accounts

#[generic_array_struct(builder destr trymap pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SyncNativeIxAccs<T> {
    /// The wSOL token account to sync
    pub acc: T,
}

impl_memset!(SyncNativeIxAccs);

pub type SyncNativeIxAccsFlag = SyncNativeIxAccs<bool>;

pub const SYNC_NATIVE_IX_IS_SIGNER: SyncNativeIxAccsFlag = SyncNativeIxAccsFlag::memset(false);

pub const SYNC_NATIVE_IX_IS_WRITABLE: SyncNativeIxAccsFlag = SyncNativeIxAccsFlag::memset(true);

// Data

pub const SYNC_NATIVE_IX_DISCM: u8 = 17;

pub const SYNC_NATIVE_IX_DATA_LEN: usize = SyncNativeIxData::LEN;

pub type SyncNativeIxData = DismOnlyIxData<SYNC_NATIVE_IX_DISCM>;
