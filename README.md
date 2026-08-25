# System stat
An easy and helpful **CLI** utility based on **clap rs**, that helps get varios data about system.

## 🖥️ Commands
### Available commands:
|   Command   |                           Explanation                      |
| ----------- | ---------------------------------------------------------- |
| `hello`     | Prints hello message                                       |
| `sys-data`  | Prints system data                                         |
| `disk-data` | Prints disk(s) data                                        |
| `cpu-data`  | Prints CPU data                                            |
| `ram-data`  | Prints RAM data                                            |
| `proc-data` | Prints process data                                        |

### Usage
#### hello
    system_stat hello

#### sys-data
    system_stat sys-data

#### disk-data
Simple usage:

    system_stat disk-data

Options:

*Usage with options*:

    system_stat disk-data --data space
    system_stat disk-data --data kind
    system_stat disk-data --help

* `--data <DATA>` - where DATA is the presentation mode (all available is **3** modes: **full**, **space** and **kind**), **full** mode is setted by default. See modes description down.
* `--help` or `-h` - get help info

*Data presentation modes*:
|  Mode   |               Explanation          |
| ------- | ---------------------------------- |
| `full`  | Show all data about disks          |
| `space` | Show only disks space data in GB   |
| `kind`  | Show only disks kinds(HDD, SSD...) |

#### cpu-data
Subcommands:

*Usage with subcommands*:

    system_stat cpu-data base-data

    system_stat cpu-data observe
    system_stat cpu-data observe --show-brand
    system_stat cpu-data observe --show-brand --show-freq

* `base-data` - shows base data about CPU
* `observe` - observes CPU cores usage data and displays it in the terminal(press **Ctrl + C** to end observing), see options description down.

`observe` *command options*:

|     Option     |        Explanation       |
| -------------- | ------------------------ |
| `--show-brand` | Show CPU core brand name |
| `--show-freq`  | Show CPU core frequency  |

#### ram-data
Subcommands:

*Usage with subcommands*:

    system_stat ram-data base-data
    system_stat ram-data observe

* `base-data` - shows base data about RAM
* `observe` - observes RAM usage data and displays it in the terminal(press **Ctrl + C** to end observing).

#### proc-data
Subcommands:

*Usage with subcommands*:

    system_stat proc-data curr-pid
    system_stat proc-data procs-by-name --proc-name <PROC_NAME>
    system_stat proc-data procs-by-name --proc-name <PROC_NAME> --exact

* `curr-pid` - shows PID of the current process
* `procs-by-name` - shows processes by specific name

Options:

* `--exact` or `-e` - show processes exactly by name
* `--help` or `-h` - get help info

## 📦 Used crates
* sysinfo
* crossterm
* clap