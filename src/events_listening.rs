use std::{
    sync::{Arc, Mutex, mpsc::Receiver},
    thread,
};

use better_sms::mutex::{MutexGuardWork, MutexWork};

use crate::{event::Event, init_services::Services};

pub fn run_events_listening(event_reciver: Receiver<Event>, services: &Arc<Mutex<Services>>) {
    let services = services.clone();

    _ = thread::spawn(move || {
        loop {
            let event = event_reciver.recv().unwrap();
            match event {
                Event::UpdateGUI => services
                    .lock_unw()
                    .use_guard(|services| services.sections_manager.draw_sections()),
            }
        }
    });
}
