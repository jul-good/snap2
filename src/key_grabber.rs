use crate::app_launcher::launch_app;
use crate::dock_reader;
use rdev::{grab, Event, EventType, Key};
use std::sync::atomic::{AtomicBool, Ordering};
use anyhow::Context;

pub fn start() -> anyhow::Result<()> {
    let right_cmd_pressed = AtomicBool::new(false);

    let callback_grab = move |event: Event| -> Option<Event> {
        match event.event_type {
            EventType::KeyPress(Key::MetaRight) => {
                right_cmd_pressed.store(true, Ordering::Relaxed);
                None
            }
            EventType::KeyRelease(Key::MetaRight) => {
                right_cmd_pressed.store(false, Ordering::Relaxed);
                None
            }
            EventType::KeyPress(key) if right_cmd_pressed.load(Ordering::Relaxed) => {
                if let Err(e) = handle_shortcut(key) {
                    println!("Unable to handle shortcut {:?}", e);
                };

                None
            }
            _ => Some(event), // Allow all other events to pass through
        }
    };

    // Start grabbing events (this will block)
    if let Err(error) = grab(callback_grab) {
        eprintln!("Error: {:?}", error);
    }

    Ok(())
}

fn handle_shortcut(key: Key) -> anyhow::Result<()> {
    let index = key_to_char(key).context("Shortcut binding not found")?;
    let dock_apps = dock_reader::read_dock_apps()?;
    let dock_app = dock_apps
        .get(index)
        .context("Dock app not found for shortcut")?;
    launch_app(dock_app.tile_data.file_label.as_str())?;

    Ok(())
}

fn key_to_char(key: Key) -> Option<usize> {
    match key {
        Key::KeyQ => Some(0),
        Key::KeyW => Some(1),
        Key::KeyE => Some(2),
        Key::KeyR => Some(3),
        Key::KeyT => Some(4),
        Key::KeyA => Some(5),
        Key::KeyS => Some(6),
        Key::KeyD => Some(7),
        Key::KeyF => Some(8),
        Key::KeyG => Some(9),
        _ => None,
    }
}
