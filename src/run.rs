use anyhow::Result;
use better_sms::{
    arc::ArcCreate,
    mutex::{MutexCreate, MutexGuardWork, MutexWork},
};
use std::sync::mpsc;

use crate::{
    events_listening::run_events_listening, init_services::init_services, sections::FilesSection,
    user_inputs::run_user_inputs_listening,
};

pub fn run() -> Result<()> {
    let (sender, reciver) = mpsc::channel();
    let sender = sender.create_mutex().create_arc();

    let services = &init_services().create_mutex().create_arc();

    run_events_listening(reciver, services);
    run_user_inputs_listening(&sender, services);

    loop {}
}
