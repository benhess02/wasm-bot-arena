#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn connect4_select_column(column: u32);
}

pub fn select_column(column: u32) {
    unsafe {
        connect4_select_column(column);
    }
}
