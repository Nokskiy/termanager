use std::sync::{Arc, Mutex};

use better_sms::{arc::ArcCreate, mutex::MutexCreate};

use crate::display::{
    display_buffer::{self, DisplayBuffer},
    sections::{self, SectionManager},
};

pub struct ServicesManager {
    pub display_buffer: Arc<Mutex<DisplayBuffer>>,
    pub sections_manager: Arc<Mutex<SectionManager>>,
}

impl ServicesManager {
    pub fn new() -> Self {
        let mut display_buffer = DisplayBuffer::new();
        display_buffer.update_grid();
        Self {
            display_buffer: display_buffer.create_mutex().create_arc(),
            sections_manager: SectionManager::new().create_mutex().create_arc(),
        }
    }
}
