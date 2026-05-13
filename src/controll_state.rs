use logger_rust::log_debug;

#[derive(Debug)]
pub enum ControllState {
    Visual,
    Normal,
    Command,
}

pub struct ControllManager {
    pub current_state: ControllState,
}

impl ControllManager {
    pub fn new() -> Self {
        Self {
            current_state: ControllState::Visual,
        }
    }

    pub fn change_state(&mut self, state: ControllState) {
        self.current_state = state;
    }
}
