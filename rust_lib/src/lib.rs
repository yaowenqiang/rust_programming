use std::os::raw::c_int;
#[unsafe(no_mangle)]
pub extern "C" fn add2(a: c_int, b: c_int) -> c_int {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn greet(name: *const u8) -> *mut i8 {
    let c_str = unsafe { std::ffi::CStr::from_ptr(name as *const i8) };
    let name_str = c_str.to_str().unwrap();
    let greeting = format!("Hello {}!", name_str);
    let c_string = std::ffi::CString::new(greeting).unwrap();
    c_string.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut u8) {
    unsafe {
        if !s.is_null() {
            let _ = std::ffi::CString::from_raw(s as *mut i8);
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
