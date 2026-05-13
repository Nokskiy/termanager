use crate::{controll_state::ControllManager, sections_manager::SectionsManager};

pub fn init_services() -> Services {
    Services {
        sections_manager: SectionsManager::new(),
        controll_manager: ControllManager::new(),
    }
}

pub struct Services {
    pub sections_manager: SectionsManager,
    pub controll_manager: ControllManager,
}
