#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerColor {
    Red,
    Black,
}

impl PlayerColor {
    pub fn from_player_index(index: usize) -> Self {
        match index {
            0 => PlayerColor::Black,
            1 => PlayerColor::Red,
            _ => unreachable!(),
        }
    }

    pub fn opposite(&self) -> PlayerColor {
        match self {
            PlayerColor::Red => PlayerColor::Black,
            PlayerColor::Black => PlayerColor::Red,
        }
    }
}

#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn connect4_select_column(column: usize);
    fn connect4_get_tile(column: usize, row: usize) -> u8;
}

pub fn select_column(column: usize) {
    unsafe {
        connect4_select_column(column);
    }
}

pub fn get_tile_state(column: usize, row: usize) -> Option<PlayerColor> {
    match unsafe { connect4_get_tile(column, row) } {
        0 => None,
        1 => Some(PlayerColor::Black),
        2 => Some(PlayerColor::Red),
        _ => unreachable!("Invalid tile"),
    }
}
