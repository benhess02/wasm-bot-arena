#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn bot_log(str: *const u8, len: usize);
}

pub fn log(str: &str) {
    unsafe { bot_log(str.as_ptr(), str.len()) };
}

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    log("Hello World!!!");
}
