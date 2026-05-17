use std::{ops::DerefMut, path::PathBuf};

use anyhow::Result;
use better_sms::{
    arc::ArcCreate,
    mutex::{MutexCreate, MutexGuardWork, MutexWork},
};
use rand::random;
use termanager::{
    display::display_buffer, services::ServicesManager,
    user_inputs::user_inputs_listener::run_user_listener,
};

pub fn main() -> Result<()> {
    run()?;
    Ok(())
}

pub fn run() -> Result<()> {
    let services_manager = ServicesManager::new().create_mutex().create_arc();

    run_user_listener(&services_manager)?;
    loop {}
}
