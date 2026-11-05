#[unsafe(no_mangle)]
pub extern "C" fn sys_print(msg: *const u8, len: usize) {
    let bytes = unsafe {
        core::slice::from_raw_parts(msg, len)
    };

    let text = match core::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => panic!("error"),
    };

    println!("{}", text);
}