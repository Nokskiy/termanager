use std::{
    any::Any,
    io::{Cursor, Write, stdout},
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    sections::{CommandsSection, FilesSection, Section, SectionType},
    sections_display_buffer::{DisplayElement, SectionsDisplayBuffer},
    sections_manager,
};
use better_sms::{
    arc::ArcCreate,
    mutex::{MutexCreate, MutexGuardWork, MutexWork},
};
use crossterm::{
    cursor, execute,
    style::{self, Print, Stylize},
    terminal,
};
use rand::{RngExt, make_rng, rng, rngs::ThreadRng};

pub struct SectionsManager {
    pub sections: Vec<Box<dyn Section + Send>>,
    pub display_buffer: SectionsDisplayBuffer,
}

impl SectionsManager {
    pub fn new() -> Self {
        Self {
            sections: vec![],
            display_buffer: SectionsDisplayBuffer::new(),
        }
    }

    pub fn draw_sections(&mut self) {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout().flush().unwrap();

        self.display_buffer.set_size();
        for section in &self.sections {
            section.add_to_buffer(&self.sections, &mut self.display_buffer);
        }

        for (y_i, y) in self.display_buffer.grid.iter().enumerate() {
            for (x_i, x) in y.iter().enumerate() {
                match x {
                    DisplayElement::Border => {
                        execute!(
                            stdout(),
                            cursor::MoveTo(x_i as u16, y_i as u16),
                            style::Print(" ".on_cyan())
                        )
                        .unwrap();
                    }
                    DisplayElement::Char(char) => {
                        execute!(
                            stdout(),
                            cursor::MoveTo(x_i as u16, y_i as u16),
                            style::Print(char.on_red())
                        )
                        .unwrap();
                    }
                    DisplayElement::Space => {}
                }
            }
        }
    }

    pub fn add_section(&mut self, section_type: SectionType) {
        match section_type {
            SectionType::FilesSection => {
                let section_id = rng().random();
                self.sections.push(FilesSection::new(section_id));
            }
            SectionType::CommandsSection => {
                let section_id = rng().random();
                self.sections.push(CommandsSection::new(section_id));
            }
        }
    }
}
