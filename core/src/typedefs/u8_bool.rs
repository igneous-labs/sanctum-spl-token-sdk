#[inline]
pub const fn try_bool_from_u8(byte: u8) -> Option<bool> {
    Some(match byte {
        0 => false,
        1 => true,
        _ => return None,
    })
}
