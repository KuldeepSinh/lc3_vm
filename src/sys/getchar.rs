use libc::c_int;
extern "C" {
    fn getchar() -> c_int;
}

/// `get_char` calls external (C) function getchar using libc.
pub fn get_char() -> i32 {
    unsafe { getchar() }
}
