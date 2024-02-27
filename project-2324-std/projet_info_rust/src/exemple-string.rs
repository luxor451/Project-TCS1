use std::ffi::CString;
use std::os::raw::c_char;

extern crate libc;

fn main() {
    let original = "simulation";
    let length = original.len();

    // declaring a string of length length + 6 characters + the final
    // \0 character
    let mut my_string = vec![0; length + 7];
    unsafe {
        libc::sprintf(
            my_string.as_mut_ptr() as *mut c_char,
            "simulation-%d.%s\0".as_ptr() as *const i8,
            3,
            "txt\0".as_ptr() as *const i8,
        );
    }

    // Convert the CString to &str
    let my_string = unsafe { CString::from_vec_unchecked(my_string) };
    let my_string = my_string.to_str().unwrap();

    println!("The new string: {}", my_string);
}