pub mod cpu;
mod panic;
mod static_cell;

use core::slice;
pub use static_cell::StaticCell;

/// Converts the given C string into a string slice
///
/// # Safety
///
/// This function assumes that `s` points to a permanently valid and null-terminated C string
pub unsafe fn cstr_to_str(s: *const i8) -> &'static str {
    let len = strlen(s);
    let sl = slice::from_raw_parts(s, len as usize + 1);
    &*(&sl[..sl.len() - 1] as *const [i8] as *const str)
}

/// Determines the length of the given C string
///
/// # Safety
///
/// This function assumes that `s` points to a valid and null-terminated C string
pub unsafe fn strlen(mut s: *const i8) -> usize {
    let begin = s;
    while *s != 0 {
        s = s.add(1);
    }
    s.offset_from(begin) as usize
}
