mod game;
mod ui;

use crate::guess::game::console;
use crate::guess::ui::ui;

pub fn start(choice: String) {
   if choice.trim() != "UI" {
      console();
  } else {
      ui();
  }
}

