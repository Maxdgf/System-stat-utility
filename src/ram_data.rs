use std::{io::{ self, stdout }, thread};

use crossterm::{
    cursor, 
    event::{ self, Event, KeyCode, KeyModifiers }, 
    execute, 
    terminal::{ self, EnterAlternateScreen, LeaveAlternateScreen }
};
use sysinfo::{ MemoryRefreshKind, RefreshKind, System };

pub struct Ram {
    pub total_ram: u64,
    pub total_swap: u64
}

pub fn get_base_ram_data() -> Ram {
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything())
    );

    return Ram {
        total_ram: system.total_memory() / 1073741824,
        total_swap: system.total_swap() / 1073741824,
    };
}

pub fn observe_ram_usage() -> Result<(), Box<io::Error>> {
    // refresh system CPU specifics
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything())
    );

    terminal::enable_raw_mode()?;              // enable terminal raw mode
    execute!(stdout(), EnterAlternateScreen)?; // enter to alternate screen in terminal

    loop {
        if event::poll(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL)? {
            if let Event::Key(key_event) = event::read()? {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                    _ => {} // nothing to do
                }
            }

        } else {
            // clear all terminal content
            execute!(
                stdout(),
                cursor::MoveTo(0, 0),
                terminal::Clear(terminal::ClearType::All)
            )?;
            
            println!("| RAM usage: {}", system.used_memory());
            println!("| RAM swap usage: {}", system.used_swap());

            thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    }

    terminal::disable_raw_mode()?; // disable terminal raw mode
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}