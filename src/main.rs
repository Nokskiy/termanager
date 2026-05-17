use std::{ops::DerefMut, path::PathBuf};

use better_sms::mutex::{MutexGuardWork, MutexWork};
use rand::random;
use termanager::{display::display_buffer, services::ServicesManager};

pub fn main() {
    run();
}

pub fn run() {
    let services_manager = ServicesManager::new();

    for _ in 0..5 {
        services_manager.sections_manager.lock_unw().create_section(
            termanager::display::sections::Section::Files(random(), PathBuf::from("D:\\")),
        );
    }

    services_manager
        .sections_manager
        .lock_unw()
        .update_display_buffer(services_manager.display_buffer.lock_unw().deref_mut());

    services_manager.display_buffer.lock_unw().draw();

    loop {}
}
