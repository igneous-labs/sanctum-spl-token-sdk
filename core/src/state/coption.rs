#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum COptionDiscm {
    None,
    Some,
}

impl COptionDiscm {
    pub const NONE: [u8; 4] = [0; 4];
    pub const SOME: [u8; 4] = [1, 0, 0, 0];
}

impl COptionDiscm {
    #[inline]
    pub const fn try_from_arr(arr: &[u8; 4]) -> Option<Self> {
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
pub const fn unpack_valid_coption<'a, T>(discm: &[u8; 4], val: &'a T) -> Option<&'a T> {
    match COptionDiscm::try_from_arr(discm) {
        Some(COptionDiscm::None) => None,
        Some(COptionDiscm::Some) => Some(val),
        // assume coption prevalidated beforehand
        None => unreachable!(),
    }
}
