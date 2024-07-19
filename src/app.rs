use std::env::current_dir;
use std::path::PathBuf;

pub enum Input_Mode {
    Normal,
    Editing,
}

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub struct App {
    pub value_input: String,
    pub input_mode: Input_Mode,
    pub current_screen: CurrentScreen,
    pub current_dir: PathBuf,
    pub character_index: usize,
}

impl App {
    pub fn new() -> App {
        App {
            value_input: String::new(),
            input_mode: Input_Mode::Normal,
            current_screen: CurrentScreen::Main,
            current_dir: current_dir().unwrap().join("todo.txt"),
            character_index: 0,
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.value_input.chars().count()) // restrict to interval b/w 0 & size of input
    }
}
