use libc::c_int;
extern "C" {
    fn getchar() -> c_int;
}

pub fn get_char() -> i32 {
    unsafe { getchar() }
}
