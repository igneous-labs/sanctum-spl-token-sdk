#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum COptionDiscm {
    None,
    Some,
}

impl COptionDiscm {
    pub const LEN: usize = 4;

    pub const NONE: [u8; Self::LEN] = [0; Self::LEN];
    pub const SOME: [u8; Self::LEN] = [1, 0, 0, 0];
}

impl COptionDiscm {
    #[inline]
    pub const fn try_from_arr(arr: &[u8; Self::LEN]) -> Option<Self> {
        Some(match *arr {
            Self::NONE => Self::None,
            Self::SOME => Self::Some,
            _ => return None,
        })
    }
}

/// Unpack a COption, assuming the discriminant has been validated beforehand
///
/// # Panics
/// - if discriminant is not valid
#[inline]
pub const fn unpack_valid_coption<'a, T>(
    discm: &[u8; COptionDiscm::LEN],
    val: &'a T,
) -> Option<&'a T> {
    match COptionDiscm::try_from_arr(discm) {
        Some(COptionDiscm::None) => None,
        Some(COptionDiscm::Some) => Some(val),
        // assume coption prevalidated beforehand
        None => unreachable!(),
    }
}
