use sysinfo::{ self, ProcessRefreshKind, RefreshKind, System };

pub struct Proc {
    pub pid: String,
    pub name: String
}

/// Returns current process PID.
pub fn get_current_process_pid() -> String {
    let current_process_pid = sysinfo::get_current_pid(); // current pid

    return match current_process_pid {
        Ok(pid) => pid.to_string(),
        Err(_) => String::from("Unknown"),
    }
}

/// Returns processes PID by name.
pub fn get_processes_pid_by_name(name: &str, is_exact: &bool) -> Vec<Proc> {
    // refresh with processes specifics
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything())
    );

    return if *is_exact {
        system.processes_by_exact_name(
            name.as_ref()).map(|proc|
            Proc {
                pid: proc.pid().to_string(),
                name: proc.name().to_str().unwrap_or("Unknown").to_string()
            }
        ).collect()
    } else {
        system.processes_by_name(
            name.as_ref()).map(|proc|
            Proc {
                pid: proc.pid().to_string(),
                name: proc.name().to_str().unwrap_or("Unknown").to_string()
            }
        ).collect()
    }
}
