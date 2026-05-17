pub struct UserControllModManager {
    pub current_mod: UserControllMod,
}

impl UserControllModManager {
    pub fn new() -> Self {
        Self {
            current_mod: UserControllMod::Visul,
        }
    }
}

pub enum UserControllMod {
    Visul,
    Command,
    Normal,
}

impl UserControllMod {
    pub fn to_string(&self) -> String {
        match self {
            Self::Visul => "Visual".to_string(),
            Self::Command => "Command".to_string(),
            Self::Normal => "Normal".to_string(),
        }
    }
}
