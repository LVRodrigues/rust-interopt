use std::ffi::{c_char};
use std::env;

#[repr(C)]
pub struct VersionText {
    text: [c_char; 512]
}

#[unsafe(no_mangle)]
pub extern "C" fn version(text: *mut VersionText) {
    let s = format!(
        "{}\n\
        \tVersion.....: {}\n\
        \tDescription.: {}\n\
        \tAuthor......: {}\n\
        \tSystem......: {}\n\
        \tArchitecture: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_AUTHORS"),
        env::consts::OS,
        env::consts::ARCH
    );

    let bytes = s.as_bytes();
    let len = bytes.len().min(511); // Reserve one for null terminator

    unsafe {
        if !text.is_null() {
            let text_ptr = (*text).text.as_mut_ptr();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), text_ptr as *mut u8, len);
            *text_ptr.add(len) = 0; // Null terminator
        }
    }
}