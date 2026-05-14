pub struct CommandsManager {
    pub cur_command: String,
    pub cursor_index: usize,
}

impl CommandsManager {
    pub fn new() -> Self {
        Self {
            cur_command: String::new(),
            cursor_index: 0,
        }
    }

    pub fn add_char(&mut self, char: char) {
        self.cur_command.insert(self.cursor_index, char);
    }
}
