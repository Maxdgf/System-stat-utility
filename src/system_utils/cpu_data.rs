use std::{ io::{self, stdout}, thread, time::Duration };

use sysinfo::{ CpuRefreshKind, RefreshKind, System };
use crossterm::{ 
    cursor, 
    event::{ self, Event, KeyCode, KeyModifiers }, 
    execute, 
    style::Stylize, 
    terminal::{ self, EnterAlternateScreen, LeaveAlternateScreen } 
};

/// CPU data.
pub struct Cpu {
    pub physical_cores_count: usize,
    pub cpus_count: usize
}

/// Returns CPU data.
pub fn get_cpu_data() -> Cpu {
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())
    );

    Cpu {
        physical_cores_count: System::physical_core_count().unwrap_or(0),
        cpus_count: system.cpus().len(),
    }
}

/// Launches the CPU cores usage observing process. It displays data until CTRL + C is pressed.
pub fn observe_cpu_data(show_brand: &bool, show_freq: &bool) -> Result<(), Box<io::Error>> {
    // refresh system CPU specifics
    let mut system = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())
    );

    terminal::enable_raw_mode()?;              // enable terminal raw mode
    execute!(stdout(), EnterAlternateScreen)?; // enter to alternate screen in terminal

    loop {
        if event::poll(Duration::from_millis(100))? {
            // read key codes
            if let Event::Key(key_event) = event::read()? {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => break, // CTRL + C
                    _ => {}                                               // nothing to do
                }
            }

        } else {
            system.refresh_cpu_usage(); // refresh CPU usage

            // clear all terminal content
            execute!(
                stdout(),
                cursor::MoveTo(0, 0),
                terminal::Clear(terminal::ClearType::All)
            )?;

            for cpu in system.cpus().iter() {
                let usage = cpu.cpu_usage().round();

                let name = if usage > 0.0 { 
                    format!("{} CPU: {}", "●".green(), cpu.name().green()) 
                } else if usage == 100.0 {
                    format!("{} CPU: {}", "●".red(), cpu.name().red()) 
                } else {  
                    format!("{} CPU: {}", "●".white(), cpu.name().white()) 
                };

                println!(
                    "| {}{}{} - usage: {}%", 
                    name, 
                    if *show_brand { format!(" - brand: {}", cpu.brand()) } else { String::new() }, 
                    if *show_freq { format!(" - freq: {}MHz", cpu.frequency()) } else { String::new() },
                    usage
                );
            }

            thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    }

    terminal::disable_raw_mode()?;             // disable terminal raw mode
    execute!(stdout(), LeaveAlternateScreen)?; // leave the alternate screen in terminal

    Ok(())
}
