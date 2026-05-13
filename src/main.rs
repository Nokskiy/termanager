use anyhow::Result;

pub mod controll_state;
pub mod event;
pub mod events_listening;
pub mod init_services;
pub mod run;
pub mod sections;
pub mod sections_manager;
pub mod user_inputs;

fn main() -> Result<()> {
    run::run()?;
    Ok(())
}
