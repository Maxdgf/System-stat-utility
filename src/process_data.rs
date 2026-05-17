use sysinfo::{self, ProcessRefreshKind, RefreshKind, System};

pub struct Proc {
    pub pid: String,
    pub name: String
}

/// Returns current process PID.
pub fn get_current_process_pid() -> String {
    let curent_process_pid = sysinfo::get_current_pid();

    match curent_process_pid {
        Ok(pid) => return pid.to_string(),
        Err(_) => return String::from("Unknown"),
    }
}

pub fn get_processes_pid_by_name(name: &str, is_exact: &bool) -> Vec<Proc> {
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything())
    );

    if *is_exact {
        return system.processes_by_exact_name(
            name.as_ref()).map(|proc| 
                Proc { 
                    pid: proc.pid().to_string(), 
                    name: proc.name().to_str().unwrap_or("Unknown").to_string()
                }
            ).collect();
    } else {
        return system.processes_by_name(
            name.as_ref()).map(|proc| 
                Proc { 
                    pid: proc.pid().to_string(), 
                    name: proc.name().to_str().unwrap_or("Unknown").to_string()
                }
            ).collect();
    }
}
