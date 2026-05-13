use std::{any::Any, io::stdout};

use crossterm::{
    cursor, execute,
    style::{self, Stylize},
};

use crate::sections_manager::SectionsManager;

pub enum SectionType {
    FilesSection,
}

pub trait Section: Any {
    fn display_section(&self, sections_manager: &SectionsManager);
    fn as_any(&self) -> &dyn Any;
    fn get_section_id(&self) -> u16;
}

pub struct FilesSection {
    section_id: u16,
}

impl FilesSection {
    pub fn new(section_id: u16) -> Box<dyn Section + Send> {
        Box::new(Self { section_id })
    }
}

impl Section for FilesSection {
    fn display_section(&self, sections_manager: &SectionsManager) {
        let (width, height) = crossterm::terminal::size().unwrap();

        let mut files_sections_count = 0;
        for i in &sections_manager.sections {
            if let Some(_) = i.as_any().downcast_ref::<FilesSection>() {
                files_sections_count += 1;
            }
        }

        let mut section_index = 0;

        for (index, section) in sections_manager.sections.iter().enumerate() {
            if let Some(sec) = section.as_any().downcast_ref::<FilesSection>() {
                if sec.get_section_id() == self.get_section_id() {
                    section_index = index as u16;
                    break;
                }
            }
        }

        let section_width = width / files_sections_count as u16;
        let start_x = section_width * section_index;
        let end_x = start_x + section_width;

        for x in start_x..end_x {
            execute!(stdout(), cursor::MoveTo(x, 0), style::Print(" ".on_cyan())).unwrap();
            if x == end_x - 1 || x == start_x {
                for y in 0..height - 1 {
                    execute!(stdout(), cursor::MoveTo(x, y), style::Print(" ".on_cyan())).unwrap();
                }
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_id(&self) -> u16 {
        self.section_id.clone()
    }
}
