use std::{io::{ self, stdout }, thread, time::Duration};

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
        if event::poll(Duration::from_millis(100))? {
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

            let ram_usage = system.used_memory() / 1073741824;    // current RAM usage in GB
            let ram_swap_usage = system.used_swap() / 1073741824; // current RAM swap usage in GB
            
            println!("| RAM usage: {} GB", ram_usage);
            println!("| RAM swap usage: {} GB", ram_swap_usage);

            thread::sleep(Duration::from_millis(250));
        }
    }

    terminal::disable_raw_mode()?;             // disable terminal raw mode
    execute!(stdout(), LeaveAlternateScreen)?; // enter to alternate screen in terminal

    Ok(())
}