mod sys_data;
mod disk_data;
mod cpu_data;
mod ram_data;

use std::io;
use clap::{ Parser, Subcommand };

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
    Sysdata,

    /// Print disk(s) data
    Diskdata {
        #[arg(long, default_value_t=String::from("full"))]
        data: String
    },

    /// Print CPU data
    Cpudata {
        /// Observe CPU cores usage data
        #[arg(short, long, default_value_t=false)]
        observe: bool,

        /// Show CPU core brand ! Uses only with --observe !
        #[arg(long, default_value_t=false)]
        show_brand: bool,

        /// Show CPU core frequency ! Uses only with --observe !
        #[arg(long, default_value_t=false)]
        show_freq: bool
    },

    /// Print RAM data
    Ramdata {
        /// Observe RAM usage data
        #[arg(short, long, default_value_t=false)]
        observe: bool
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
        Some(Command::Sysdata) => {
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
            println!("|");
        }
        Some(Command::Diskdata { data }) => {
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
                        println!("|");
                    }

                    println!("| # total disks count: {}", disks_data.len());
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
                        println!("|");
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
                        println!("|");
                    }
                }
                _ => println!("ERROR, unknown argument: {}", data)
            }
        }
        Some(Command::Cpudata { observe, show_brand, show_freq }) => {
            if *observe {
                cpu_data::observe_cpu_data(show_brand, show_freq)?; // launch observing CPU data
            } else {
                let cpu_data = cpu_data::get_cpu_data();
                
                println!("| > CPU data");
                println!("|");
                println!("|\\_______________________");
                println!("| - CPU's count:          {}", cpu_data.cpus_count);
                println!("| - physical cores count: {}", cpu_data.physical_cores_count);
                println!("|/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾");
                println!("|");

            }
        }
        Some(Command::Ramdata { observe }) => {
            if *observe {
                ram_data::observe_ram_usage()?; // launch obseving RAM data
            } else {
                let ram_base_data = ram_data::get_base_ram_data();

                println!("| > RAM data");
                println!("|");
                println!("|\\_______________________");
                println!("| - total RAM:            {}", ram_base_data.total_ram);
                println!("| - total RAM swap:       {}", ram_base_data.total_swap);
                println!("|/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾");
                println!("|");
            }
        }
        None => println!("ERROR, unknown command.")
    }

    Ok(())
}
