#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AccountState {
    Initialized,
    Frozen,
}

impl AccountState {
    pub const INITIALIZED: u8 = 1;
    pub const FROZEN: u8 = 2;

    #[inline]
    pub const fn into_byte(self) -> u8 {
        match self {
            Self::Initialized => Self::INITIALIZED,
            Self::Frozen => Self::FROZEN,
        }
    }

    #[inline]
    pub const fn try_from_byte(byte: u8) -> Option<Self> {
        Some(match byte {
            Self::INITIALIZED => Self::Initialized,
            Self::FROZEN => Self::Frozen,
            _ => return None,
        })
    }
}
