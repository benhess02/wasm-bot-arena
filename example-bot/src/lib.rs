use std::mem;

mod bot;
mod connect4;

use bot::Bot;

struct ExampleBot {
    name: String,
}

impl Bot for ExampleBot {
    fn init() -> Self {
        bot::log("INIT");
        Self {
            name: String::from("Bot"),
        }
    }

    fn update(&mut self) {
        bot::log(&format!("UPDATE: {}", self.name));
        connect4::select_column(0);
    }
}

bot!(ExampleBot);
