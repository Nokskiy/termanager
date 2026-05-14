use std::{
    io::stdout,
    sync::{Arc, Mutex, mpsc::Sender},
    thread,
};

use better_sms::mutex::{MutexGuardWork, MutexWork};
use crossterm::{
    cursor::SetCursorStyle,
    event::{KeyCode, KeyModifiers},
    execute, terminal,
};

use crate::{event::Event, init_services::Services};

pub fn run_user_inputs_listening(
    sender: &Arc<Mutex<Sender<Event>>>,
    services: &Arc<Mutex<Services>>,
) {
    let services = services.clone();
    let sender = sender.clone();

    terminal::enable_raw_mode().unwrap();

    thread::spawn(move || {
        while let Ok(crossterm_event) = crossterm::event::read() {
            if let crossterm::event::Event::Key(key_event) = crossterm_event {
                if key_event.kind == crossterm::event::KeyEventKind::Release {
                    continue;
                }

                let normal_mod = || match key_event.code {
                    KeyCode::Char('h') => println!("h"),
                    KeyCode::Char('j') => println!("j"),
                    KeyCode::Char('k') => println!("k"),
                    KeyCode::Char('l') => println!("l"),

                    _ => {}
                };

                let visual_mod = || match key_event.code.to_string().to_lowercase().trim() {
                    ":" => {
                        services
                            .lock_unw()
                            .controll_manager
                            .change_state(crate::controll_state::ControllState::Command);
                    }
                    "a" => {
                        services
                            .lock_unw()
                            .controll_manager
                            .change_state(crate::controll_state::ControllState::Normal);
                    }
                    _ => {}
                };

                let command_mod = || {
                    if key_event.code == KeyCode::Enter {}

                    let char = key_event.code.to_string().to_lowercase();
                };

                if key_event.code == KeyCode::Esc {
                    services
                        .lock_unw()
                        .controll_manager
                        .change_state(crate::controll_state::ControllState::Visual);
                }

                if key_event.code == KeyCode::Tab {
                    std::process::exit(0);
                }

                let services_guard = services.lock_unw();
                match services_guard.controll_manager.current_state {
                    crate::controll_state::ControllState::Visual => {
                        drop(services_guard);
                        visual_mod();
                        execute!(stdout(), SetCursorStyle::BlinkingUnderScore).unwrap();
                    }
                    crate::controll_state::ControllState::Normal => {
                        drop(services_guard);
                        normal_mod();
                        execute!(stdout(), SetCursorStyle::BlinkingBlock).unwrap();
                    }
                    crate::controll_state::ControllState::Command => {
                        drop(services_guard);
                        command_mod();
                        execute!(stdout(), SetCursorStyle::BlinkingBlock).unwrap();
                    }
                }

                sender
                    .lock_unw()
                    .use_guard(|sender| sender.send(Event::UpdateGUI).unwrap());
            }
        }
        terminal::disable_raw_mode().unwrap();
    });
}
