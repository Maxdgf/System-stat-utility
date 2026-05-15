use sysinfo::System;

pub struct SysData {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String
}

/// Returns system data.
pub fn get_sys_data() -> SysData {
    let name = System::name();                     // system name
    let kernel_version = System::kernel_version(); // system kernel version
    let os_version = System::os_version();         // os version
    let host_name = System::host_name();           // system host name

    return SysData {
        name: name.unwrap_or(String::from("Unknown")),
        kernel_version: kernel_version.unwrap_or(String::from("Unknown")),
        os_version: os_version.unwrap_or(String::from("Unknown")),
        host_name: host_name.unwrap_or(String::from("Unknown"))
    };
}