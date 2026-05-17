use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use better_sms::mutex::MutexWork;
use rand::random;

use crate::{
    display::display_buffer::DisplayBuffer,
    services::{self, ServicesManager},
};

pub struct SectionManager {
    pub sections: Vec<Section>,
}

impl SectionManager {
    pub fn new() -> Self {
        let mut sections = vec![];

        let id: u16 = random();
        sections.push(Section::Commands(id));

        SectionManager { sections }
    }

    pub fn create_section(&mut self, section_type: Section) {
        self.sections.push(section_type);
    }

    pub fn update_display_buffer(
        &mut self,
        services: &ServicesManager,
        display_buffer: &mut DisplayBuffer,
    ) {
        for section in &self.sections {
            match section {
                Section::Files(_, _) => {
                    Self::add_files_section_to_display(&self.sections, display_buffer)
                }
                Section::Commands(_) => {
                    Self::add_commands_section_to_display(&services, display_buffer);
                }
            }
        }
    }

    fn add_commands_section_to_display(
        services: &ServicesManager,
        display_buffer: &mut DisplayBuffer,
    ) {
        let (_, terminal_height) = display_buffer.get_size();

        let user_mod_name = services
            .user_controll_mod_manager
            .lock_unw()
            .current_mod
            .to_string();

        for (x_i, char) in user_mod_name.chars().into_iter().enumerate() {
            display_buffer.set_cell(
                x_i as usize,
                (terminal_height - 1) as usize,
                super::display_buffer::DisplayBufferElement::Char(char),
            );
        }
    }

    fn add_files_section_to_display(sections: &Vec<Section>, display_buffer: &mut DisplayBuffer) {
        let (terminal_width, terminal_height) = display_buffer.get_size();
        let files_section_height = terminal_height - 1;

        let files_sections: Vec<&Section> = sections
            .iter()
            .filter(|section| matches!(section, Section::Files(_, _)))
            .collect();
        for section in &files_sections {
            let current_section_id = section.get_id();

            let position_in_files_sections = files_sections
                .iter()
                .position(|section| section.get_id() == current_section_id);

            let files_section_width = terminal_width / files_sections.len() as u16;

            let start_x = files_section_width * position_in_files_sections.unwrap() as u16;
            let end_x = start_x + files_section_width;

            for y in 0..files_section_height {
                for x in start_x..end_x {
                    if y == 0 || y == files_section_height - 1 || x == start_x || x == end_x - 1 {
                        display_buffer.set_cell(
                            x as usize,
                            y as usize,
                            super::display_buffer::DisplayBufferElement::Border,
                        );
                    }
                }
            }
        }
    }
}

pub enum Section {
    Files(u16, PathBuf), // id, path
    Commands(u16),       // id
}

impl Section {
    fn get_id(&self) -> u16 {
        match self {
            Self::Files(id, _) => *id,
            Self::Commands(id) => *id,
        }
    }
}
