use std::mem;

mod bot;
mod connect4;

use bot::Bot;
use connect4::PlayerColor;

struct ExampleBot {
    color: PlayerColor,
}

struct Board {
    column_counts: [usize; 8],
    tiles: [[Option<PlayerColor>; 6]; 8],
}

impl Board {
    fn new() -> Self {
        Self {
            column_counts: [0; 8],
            tiles: [[None; 6]; 8],
        }
    }

    fn set_tile(&mut self, column: usize, row: usize, tile: Option<PlayerColor>) {
        if tile.is_some() {
            self.tiles[column][row] = tile;
            if self.column_counts[column] < row + 1 {
                self.column_counts[column] = row + 1;
            }
        }
    }

    fn do_move(&mut self, column: usize, color: PlayerColor) -> bool {
        let row = self.column_counts[column];
        if row > 5 {
            return false;
        }
        self.tiles[column][row] = Some(color);
        self.column_counts[column] = row + 1;
        return true;
    }

    fn undo_move(&mut self, column: usize) -> bool {
        let row = self.column_counts[column];
        if row == 0 {
            return false;
        }
        self.tiles[column][row] = None;
        self.column_counts[column] = row - 1;
        return true;
    }
}

fn get_tile_score(player: PlayerColor, tile: Option<PlayerColor>) -> f32 {
    if let Some(c) = tile {
        if c == player {
            return 1.;
        } else {
            return -1.;
        }
    } else {
        return 0.;
    }
}

fn eval(board: &Board, player: PlayerColor) -> f32 {
    let mut score = 0.0;
    for col in 0..8 {
        for row in 0..6 {
            if board.tiles[col][row].is_some() {
                continue;
            }
            if col > 0 {
                score += get_tile_score(player, board.tiles[col - 1][row]);
                if row > 0 {
                    score += get_tile_score(player, board.tiles[col - 1][row - 1]);
                }
                if row < 5 {
                    score += get_tile_score(player, board.tiles[col - 1][row + 1]);
                }
            }
            if col < 7 {
                score += get_tile_score(player, board.tiles[col + 1][row]);
                if row > 0 {
                    score += get_tile_score(player, board.tiles[col + 1][row - 1]);
                }
                if row < 5 {
                    score += get_tile_score(player, board.tiles[col + 1][row + 1]);
                }
            }
            if row > 0 {
                score += get_tile_score(player, board.tiles[col][row - 1]);
            }
            if row < 5 {
                score += get_tile_score(player, board.tiles[col][row + 1]);
            }
        }
    }
    return score;
}

fn minimax(board: &mut Board, player: PlayerColor, depth: usize) -> (usize, f32) {
    let mut best_col = 0;
    let mut best_score = f32::NEG_INFINITY;
    for col in 0..8 {
        if !board.do_move(col, player) {
            continue;
        }
        let mut score;
        if depth == 0 {
            score = eval(board, player.opposite());
        } else {
            (_, score) = minimax(board, player.opposite(), depth - 1);
        }
        board.undo_move(col);
        score *= -1.;

        if score > best_score {
            best_score = score;
            best_col = col;
        }
    }
    return (best_col, best_score);
}

impl Bot for ExampleBot {
    fn init(player_index: usize) -> Self {
        bot::log(&format!("INIT PLAYER {}", player_index));
        Self {
            color: PlayerColor::from_player_index(player_index),
        }
    }

    fn update(&mut self) {
        bot::log(&format!("UPDATE PLAYER {:?}", self.color));
        let mut board = Board::new();
        for col in 0..8 {
            for row in 0..6 {
                board.set_tile(col, row, connect4::get_tile_state(col, row));
            }
        }
        bot::log("Running minimax...");
        let (col, score) = minimax(&mut board, self.color, 3);
        bot::log(&format!("Column {}, score {}", col, score));
        connect4::select_column(col);
    }
}

bot!(ExampleBot);
