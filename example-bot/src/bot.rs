#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn bot_log(message: *const u8, len: usize);
}

pub fn log(message: &str) {
    unsafe { bot_log(message.as_ptr(), message.len()) };
}

pub trait Bot {
    fn init(player_index: usize) -> Self;
    fn update(&mut self);
}

#[macro_export]
macro_rules! bot {
    ($name:ty) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn bot_init(player_index: usize) -> *mut () {
            let boxed = Box::new(<$name>::init(player_index));
            unsafe { mem::transmute(Box::into_raw(boxed)) }
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn bot_update(state: *mut ()) {
            unsafe {
                let ptr: *mut $name = mem::transmute(state);
                (*ptr).update();
            }
        }
    };
}
