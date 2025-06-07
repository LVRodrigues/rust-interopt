

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