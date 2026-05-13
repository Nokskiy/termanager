use std::sync::{Arc, Mutex};

use crate::sections::Section;
use better_sms::{
    arc::ArcCreate,
    mutex::{MutexCreate, MutexGuardWork, MutexWork},
};

pub struct SectionsManager {
    pub sections: Vec<Box<dyn Section + Send>>,
}

impl SectionsManager {
    pub fn new() -> Self {
        Self { sections: vec![] }
    }

    pub fn draw_sections(&self) {
        for section in &self.sections {
            section.display_section();
        }
    }

    pub fn add_section(&mut self, section: Box<dyn Section + Send>) {
        self.sections.push(section);
    }
}
