use std::ffi::{c_char};

#[repr(C)]
pub struct User {
    pub uuid: [u8; 16],
    pub name: [u8; 32],
    pub count: u64, 
}

#[unsafe(no_mangle)]
pub extern "C" fn write_comment(user: &mut User, comment: *const u8, comment_len: usize) {
    let comment = unsafe { std::slice::from_raw_parts(comment, comment_len) };
    let comment_str = unsafe { std::str::from_utf8_unchecked(comment) };
    println!("{:x?} says: {}.", user.uuid.as_slice(), comment_str);
    user.count += 1;
}

#[unsafe(no_mangle)]
pub extern "C" fn version(text: *mut c_char, size: usize) {
    let str = format!("{}\n\t{}\n\t{}\n\t{}", 
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_AUTHORS"));

    let c_len = str.as_bytes().len();
    // if c_len < size {
    //     panic!();
    // }
    unsafe { std::ptr::copy(str.as_bytes().as_ptr().cast(), text, c_len) };
}