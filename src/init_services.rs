use crate::{controll_state::ControllManager, sections_manager::SectionsManager};

pub fn init_services() -> Services {
    let mut sections_manager = SectionsManager::new();
    sections_manager.add_section(crate::sections::SectionType::FilesSection);

    Services {
        sections_manager,
        controll_manager: ControllManager::new(),
    }
}

pub struct Services {
    pub sections_manager: SectionsManager,
    pub controll_manager: ControllManager,
}
