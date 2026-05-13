use std::{
    any::Any,
    io::{Write, stdout},
    sync::{Arc, Mutex},
    thread,
};

use crate::sections::{FilesSection, Section, SectionType};
use better_sms::{
    arc::ArcCreate,
    mutex::{MutexCreate, MutexGuardWork, MutexWork},
};
use crossterm::{execute, terminal};
use rand::{RngExt, make_rng, rng, rngs::ThreadRng};

pub struct SectionsManager {
    pub sections: Vec<Box<dyn Section + Send>>,
}

impl SectionsManager {
    pub fn new() -> Self {
        Self { sections: vec![] }
    }

    pub fn draw_sections(&self) {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout().flush().unwrap();
        for section in &self.sections {
            section.display_section(self);
        }
    }

    pub fn add_section(&mut self, section_type: SectionType) {
        match section_type {
            SectionType::FilesSection => {
                let section_id = rng().random();
                self.sections.push(FilesSection::new(section_id));
            }
        }
    }
}
