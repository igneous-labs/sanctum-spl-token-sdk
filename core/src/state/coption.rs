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
