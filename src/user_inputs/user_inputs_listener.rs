use std::{
    ops::DerefMut,
    process::exit,
    sync::{Arc, Mutex},
    thread,
};

use anyhow::Result;
use better_sms::mutex::MutexWork;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::enable_raw_mode,
};

use crate::{
    display::display_buffer,
    services::{self, ServicesManager},
};

pub fn run_user_listener(services: &Arc<Mutex<ServicesManager>>) -> Result<()> {
    enable_raw_mode()?;

    let services = services.clone();

    thread::spawn(move || {
        loop {
            if let Event::Key(event) = event::read().unwrap() {
                if event.code == KeyCode::End {
                    exit(0);
                } else if event.code == KeyCode::Esc {
                    services
                        .lock_unw()
                        .user_controll_mod_manager
                        .lock_unw()
                        .current_mod = super::user_controll_mod::UserControllMod::Visul;
                }

                {
                    let s_g = services.lock_unw();
                    let mut guard = s_g.user_controll_mod_manager.lock_unw();
                    if let super::user_controll_mod::UserControllMod::Visul = guard.current_mod {
                        if event.code.to_string().to_lowercase() == ':'.to_string() {
                            guard.current_mod = super::user_controll_mod::UserControllMod::Command;
                        }
                        if event.code.to_string().to_lowercase() == 'a'.to_string() {
                            guard.current_mod = super::user_controll_mod::UserControllMod::Normal;
                        }
                    }
                }

                let services_guard = services.lock_unw();
                let mut display_buffer = services_guard.display_buffer.lock_unw();
                display_buffer.update_grid();
                let mut sections_guard = services_guard.sections_manager.lock_unw();
                sections_guard.update_display_buffer(&services_guard, &mut display_buffer);

                display_buffer.draw();
            }
        }
    });

    Ok(())
}
