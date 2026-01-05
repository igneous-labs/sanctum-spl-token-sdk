macro_rules! impl_memset {
    ($Gas:ident) => {
        impl<T: Copy> $Gas<T> {
            #[inline]
            pub const fn memset(val: T) -> Self {
                Self([val; _])
            }
        }
    };
}
pub(crate) use impl_memset;

// This does not seem to produce different bytecode
// on-chain compared to .copy_from_slice(), but it allows us to retain `const`
/// caba = `const_assign_byte_array`
pub(crate) const fn caba<const A: usize, const START: usize, const LEN: usize>(
    mut arr: [u8; A],
    val: &[u8; LEN],
) -> [u8; A] {
    const {
        assert!(START + LEN <= A);
    }

    let mut i = 0;
    while i < LEN {
        arr[START + i] = val[i];
        i += 1;
    }
    arr
}

/// csba = `const_split_byte_array`
#[inline]
pub(crate) const fn csba<const M: usize, const N: usize, const X: usize>(
    data: &[u8; M],
) -> (&[u8; N], &[u8; X]) {
    const {
        assert!(N <= M);
        assert!(X == M - N)
    }

    // Safety: bounds checked above
    let (a, b) = unsafe { data.split_at_unchecked(N) };

    // SAFETY: data is guaranteed to be of length M
    // and we are splitting it into two slices of length N and X (i.e M-N)
    (unsafe { &*a.as_ptr().cast::<[u8; N]>() }, unsafe {
        &*b.as_ptr().cast::<[u8; X]>()
    })
}

/// Returns `None` if discm does not match first byte, Some(rest of data) otherwise
#[inline]
pub(crate) const fn discm_checked<const M: usize, const D: usize>(
    expected_discm: u8,
    data: &[u8; M],
) -> Option<&[u8; D]> {
    let ([discm], data) = csba::<M, 1, D>(data);
    if *discm != expected_discm {
        return None;
    }
    Some(data)
}

pub const DISCM_ONLY_IX_DATA_LEN: usize = 1;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DismOnlyIxData<const DISCM: u8>;

impl<const DISCM: u8> DismOnlyIxData<DISCM> {
    pub const DATA: u8 = DISCM;
    pub const LEN: usize = DISCM_ONLY_IX_DATA_LEN;

    #[inline]
    pub const fn as_buf() -> &'static [u8; DISCM_ONLY_IX_DATA_LEN] {
        &[Self::DATA]
    }
}

pub const U64_IX_DATA_LEN: usize = 9;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct U64IxData<const DISCM: u8>([u8; U64_IX_DATA_LEN]);

impl<const DISCM: u8> U64IxData<DISCM> {
    pub const LEN: usize = U64_IX_DATA_LEN;

    #[inline]
    pub const fn new(arg: u64) -> Self {
        const A: usize = U64_IX_DATA_LEN;

        let mut res = [0; A];

        res = caba::<A, 0, 1>(res, &[DISCM]);
        res = caba::<A, 1, 8>(res, &arg.to_le_bytes());

        Self(res)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; U64_IX_DATA_LEN] {
        &self.0
    }

    #[inline]
    pub const fn parse_no_discm(data: &[u8; 8]) -> u64 {
        u64::from_le_bytes(*data)
    }

    /// Returns `None` if discm does not match
    #[inline]
    pub const fn parse(data: &[u8; U64_IX_DATA_LEN]) -> Option<u64> {
        match discm_checked(DISCM, data) {
            None => None,
            Some(d) => Some(Self::parse_no_discm(d)),
        }
    }
}

pub const AMT_CHECKED_IX_DATA_LEN: usize = 10;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AmtCheckedIxData<const DISCM: u8>([u8; AMT_CHECKED_IX_DATA_LEN]);

impl<const DISCM: u8> AmtCheckedIxData<DISCM> {
    pub const LEN: usize = AMT_CHECKED_IX_DATA_LEN;

    #[inline]
    pub const fn new(amt: u64, decimals: u8) -> Self {
        const A: usize = AMT_CHECKED_IX_DATA_LEN;

        let mut res = [0; A];

        res = caba::<A, 0, 1>(res, &[DISCM]);
        res = caba::<A, 1, 8>(res, &amt.to_le_bytes());
        res = caba::<A, 9, 1>(res, &[decimals]);

        Self(res)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; AMT_CHECKED_IX_DATA_LEN] {
        &self.0
    }

    #[inline]
    pub const fn parse_no_discm(data: &[u8; 8]) -> u64 {
        u64::from_le_bytes(*data)
    }

    /// Returns `None` if discm does not match
    #[inline]
    pub const fn parse(data: &[u8; AMT_CHECKED_IX_DATA_LEN]) -> Option<u64> {
        match discm_checked(DISCM, data) {
            None => None,
            Some(d) => Some(Self::parse_no_discm(d)),
        }
    }
}

pub const ADDR_IX_DATA_LEN: usize = 33;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddrIxData<const DISCM: u8>([u8; ADDR_IX_DATA_LEN]);

impl<const DISCM: u8> AddrIxData<DISCM> {
    pub const LEN: usize = ADDR_IX_DATA_LEN;

    #[inline]
    pub const fn new(addr: &[u8; 32]) -> Self {
        const A: usize = ADDR_IX_DATA_LEN;

        let mut res = [0; A];

        res = caba::<A, 0, 1>(res, &[DISCM]);
        res = caba::<A, 1, 32>(res, addr);

        Self(res)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; ADDR_IX_DATA_LEN] {
        &self.0
    }

    /// Returns `None` if discm does not match
    #[inline]
    pub const fn parse(data: &[u8; ADDR_IX_DATA_LEN]) -> Option<&[u8; 32]> {
        discm_checked(DISCM, data)
    }
}
