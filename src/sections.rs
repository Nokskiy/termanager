use std::{any::Any, io::stdout};

use crossterm::{
    cursor, execute,
    style::{self, Stylize},
};

use crate::{sections_display_buffer::SectionsDisplayBuffer, sections_manager::SectionsManager};

pub enum SectionType {
    FilesSection,
    CommandsSection,
}

pub trait Section: Any {
    fn add_to_buffer(
        &self,
        sections: &Vec<Box<dyn Section + Send>>,
        displa_buffer: &mut SectionsDisplayBuffer,
    );
    fn as_any(&self) -> &dyn Any;
    fn get_section_id(&self) -> u16;
}

pub struct FilesSection {
    section_id: u16,
}

pub struct CommandsSection {
    section_id: u16,
}

impl FilesSection {
    pub fn new(section_id: u16) -> Box<dyn Section + Send> {
        Box::new(Self { section_id })
    }
}

impl CommandsSection {
    pub fn new(section_id: u16) -> Box<dyn Section + Send> {
        Box::new(Self { section_id })
    }
}

impl Section for FilesSection {
    fn add_to_buffer(
        &self,
        sections: &Vec<Box<dyn Section + Send>>,
        display_buffer: &mut SectionsDisplayBuffer,
    ) {
        let (width, height) = crossterm::terminal::size().unwrap();

        let files_sections: Vec<&FilesSection> = sections
            .iter()
            .filter_map(|s| s.as_any().downcast_ref::<FilesSection>())
            .collect();

        let files_sections_count = files_sections.len() as u16;

        let section_index = files_sections
            .iter()
            .position(|sec| sec.get_section_id() == self.get_section_id())
            .unwrap_or(0) as u16;

        if files_sections_count == 0 {
            return;
        }
        let section_width = width / files_sections_count as u16;
        let start_x = section_width * section_index;
        let end_x = start_x + section_width;

        for x in start_x..end_x {
            display_buffer.fill(
                x as usize,
                0,
                crate::sections_display_buffer::DisplayElement::Border,
            );

            if x == end_x - 1 || x == start_x {
                for y in 0..height - 2 {
                    display_buffer.fill(
                        x as usize,
                        y as usize,
                        crate::sections_display_buffer::DisplayElement::Border,
                    );
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

impl Section for CommandsSection {
    fn add_to_buffer(
        &self,
        _sections: &Vec<Box<dyn Section + Send>>,
        display_buffer: &mut SectionsDisplayBuffer,
    ) {
        let (width, height) = crossterm::terminal::size().unwrap();

        for x in 0..width {
            display_buffer.fill(
                x as usize,
                (height - 2) as usize,
                crate::sections_display_buffer::DisplayElement::Border,
            );
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_id(&self) -> u16 {
        self.section_id.clone()
    }
}
