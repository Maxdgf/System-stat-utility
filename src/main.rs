mod system_utils;

use std::io;
use clap::{ Parser, Subcommand };

use crate::system_utils::{ cpu_data, sys_data, ram_data, process_data, disk_data };

/// System stat is easy and helpful util, that helps you get varios data about your system.                                       
#[derive(Parser)]
#[command(version="1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>
}

#[derive(Subcommand)]
enum Command {
    /// Print hello message
    Hello,

    /// Print system data
    SysData,

    /// Print disk(s) data
    DiskData {
        /// Data presentation mode
        #[arg(long, default_value_t=String::from("full"))]
        data: String
    },

    /// Print CPU data
    CpuData {
        #[command(subcommand)]
        cpu_data_subcommand: Option<CpuDataSubCommand>
    },

    /// Print RAM data
    RamData {
        #[command(subcommand)]
        ram_data_subcommand: Option<RamDataSubCommand>
    },

    /// Print process data
    ProcData {
        #[command(subcommand)]
        process_data_subcommand: Option<ProcDataSubCommand>
    }
}

#[derive(Subcommand)]
enum CpuDataSubCommand {
    /// Show base CPU data
    BaseData,

    /// Observe CPU cores usage data
    Observe {
        /// Show CPU core brand
        #[arg(long, default_value_t=false)]
        show_brand: bool,
        
        /// Show CPU core frequency
        #[arg(long, default_value_t=false)]
        show_freq: bool
    }
}

#[derive(Subcommand)]
enum RamDataSubCommand {
    /// Show RAM base data
    BaseData,

    /// Observe RAM usage data
    Observe
}

#[derive(Subcommand)]
enum ProcDataSubCommand {
    /// Show current process PID
    CurrPid,

    /// Show processes by name
    ProcsByName {
        /// Show processes by name
        #[arg(long)]
        proc_name: String,
        
        /// Show proceses exactly by name
        #[arg(short, long, default_value_t=false)]
        exact: bool
    }
}

fn main() -> Result<(), Box<io::Error>> {
    let args = Cli::parse(); // parse args

    // match parsed commands
    match &args.command {
        Some(Command::Hello) => {
            // print hello message
            println!(
r" _____           _                       _        _   
/  ___|         | |                     | |      | |  
\ `--. _   _ ___| |_ ___ _ __ ___    ___| |_ __ _| |_ 
 `--. \ | | / __| __/ _ \ '_ ` _ \  / __| __/ _` | __|
/\__/ / |_| \__ \ ||  __/ | | | | | \__ \ || (_| | |_ 
\____/ \__, |___/\__\___|_| |_| |_| |___/\__\__,_|\__|
        __/ |                                         
       |___/                                    v1.0.0          
                                   
System stat is easy and helpful util, that helps you get varios data about your system. 
- Developed by Maxdgf https://github.com/Maxdgf

For get help type: system_stat --help"
            )
        }
        Some(Command::SysData) => {
            let systemdata = sys_data::get_sys_data();
            
            // print system data
            println!("| > System data");
            println!("|");
            println!("|\\________________");
            println!("| - name:           {}", systemdata.name);
            println!("| - host name:      {}", systemdata.host_name);
            println!("| - kernel version: {}", systemdata.kernel_version);
            println!("| - OS version:     {}", systemdata.os_version);
            println!("|/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾");
        }
        Some(Command::DiskData { data }) => {
            // match 'data' arg value - full, space and kind
            match data.as_str() {
                "full" => {
                    let disks_data = disk_data::get_all_disks_data();

                    // print full data about disks
                    println!("| > Disks data");
                    println!("|");

                    for data in &disks_data {
                        println!("|\\_________________");
                        println!("| - name:            {}", data.name);
                        println!("| - mount point:     {}", data.mount_point);
                        println!("| - file system:     {}", data.file_system);
                        println!("| - read-only:       {}", data.is_read_only);
                        println!("| - removable:       {}", data.is_removable);
                        println!("| - available space: {} GB", data.available_space);
                        println!("| - total space:     {} GB", data.total_space);
                        println!("| - used space:      {} GB", data.used_space);
                        println!("| - kind:            {}", data.kind);
                        println!("|/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾");
                    }

                    print!("| # total disks count: {}", disks_data.len());
                }
                "space" => { 
                    let disks_space = disk_data::get_disks_space_data();

                    // print disks data about space
                    println!("| > Disks space");
                    println!("|");

                    for data in &disks_space {
                        println!("|\\_________________");
                        println!("| - name:            {}", data.name);
                        println!("| - available space: {} GB", data.available_space);
                        println!("| - total space:     {} GB", data.total_space);
                        println!("| - used space:      {} GB", data.used_space);
                        println!("|/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾");
                    }
                }
                "kind" => {
                    let disks_data = disk_data::get_disks_kind();

                    // print disks data about disk kind
                    println!("| > Disks kind");
                    println!("|");

                    for data in &disks_data {
                        println!("|\\_________________");
                        println!("| - name:            {}", data.name);
                        println!("| - kind:            {}", data.kind);
                        println!("|/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾");
                    }
                }
                _ => {} // nothing to do
            }
        }
        Some(Command::CpuData { cpu_data_subcommand }) => {
            // match cpu data subcommands
            match cpu_data_subcommand {
                Some(CpuDataSubCommand::BaseData) => {
                    let cpu_data = cpu_data::get_cpu_data();
                
                    println!("| > CPU data");
                    println!("|");
                    println!("|\\_______________________");
                    println!("| - CPU's count:          {}", cpu_data.cpus_count);
                    println!("| - physical cores count: {}", cpu_data.physical_cores_count);
                    println!("|/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾");
                }
                Some(CpuDataSubCommand::Observe { show_brand, show_freq }) => 
                    cpu_data::observe_cpu_data(show_brand, show_freq)?, // launch observing CPU data
                None => {} // nothing to do
            }
        }
        Some(Command::RamData { ram_data_subcommand }) => {
            // match ram data subcommands
            match ram_data_subcommand {
                Some(RamDataSubCommand::BaseData) => {
                    let ram_base_data = ram_data::get_base_ram_data();

                    println!("| > RAM data");
                    println!("|");
                    println!("|\\_______________________");
                    println!("| - total RAM:            {:.2} GB", ram_base_data.total_ram);
                    println!("| - total RAM swap:       {:.2} GB", ram_base_data.total_swap);
                    println!("|/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾");
                },
                Some(RamDataSubCommand::Observe) =>
                    ram_data::observe_ram_usage()?, // launch obseving RAM data,
                None => {}
            }
        }
        Some(Command::ProcData { process_data_subcommand }) => {
            // match process data subcommands
            match process_data_subcommand {
                Some(ProcDataSubCommand::CurrPid) => {
                    let current_pid = process_data::get_current_process_pid();
                    print!("Current process PID: {}", current_pid);
                }
                Some(ProcDataSubCommand::ProcsByName { proc_name, exact }) => {
                    let processes_by_name = process_data::get_processes_pid_by_name(proc_name, exact); // proceses by name

                    println!("| > Processes by name");

                    // print processes with PID and name
                    if !processes_by_name.is_empty() {
                        println!("| Found processes: {}", processes_by_name.len());
                        for process in processes_by_name {
                            println!("| PID: {} - name: {}", process.pid, process.name);
                        }
                    } else {
                        print!("| Processes by name: '{}' not found.", proc_name);
                    }
                }
                None => {},
            }
        }
        None => {} // nothing to do
    }

    Ok(())
}
