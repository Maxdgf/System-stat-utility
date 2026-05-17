use sysinfo::Disks;

/// Full disk data.
pub struct Disk {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub is_read_only: bool,
    pub is_removable: bool,
    pub available_space: u64,
    pub total_space: u64,
    pub used_space: u64,
    pub kind: String
}

/// Disk available and total space data.
pub struct DiskSpace {
    pub name: String,
    pub available_space: u64,
    pub total_space: u64,
    pub used_space: u64
}

/// Disk kind data.
pub struct DiskKind {
    pub name: String,
    pub kind: String
}

/// Returns data about all disks.
pub fn get_all_disks_data() -> Vec<Disk> {
    let disks = Disks::new_with_refreshed_list();

    let mut disks_data: Vec<Disk> = Vec::new();
    for disk in &disks {
        let name = disk.name().to_str();                  // disk name
        let mount_point = disk.mount_point().to_str();    // disk mount point
        let file_system = disk.file_system().to_str();    // disk file system
        let is_read_only = disk.is_read_only();                   // is read only flag
        let is_removable = disk.is_removable();                   // is removable flag
        let available_space = disk.available_space() / 1073741824; // disk available space in GB
        let total_space = disk.total_space() / 1073741824;         // disk total space in GB
        let kind = disk.kind().to_string();                     // disk king
        
        // push disk data to result vector
        disks_data.push(
            Disk {
                name: name.unwrap_or("Unknown").to_string(),
                mount_point: mount_point.unwrap_or("Unknown").to_string(),
                file_system: file_system.unwrap_or("Unknown").to_string(),
                is_read_only: is_read_only,
                is_removable: is_removable,
                available_space: available_space,
                total_space: total_space,
                used_space: total_space - available_space,
                kind: kind
            }
        );
    }

    return disks_data;
}

/// Returns all disks space data.
pub fn get_disks_space_data() -> Vec<DiskSpace> {
    let disks = Disks::new_with_refreshed_list();

    let mut disks_space: Vec<DiskSpace> = Vec::new();
    for disk in &disks {
        let name = disk.name().to_str();                  // disk name
        let available_space = disk.available_space() / 1073741824; // disk available space in GB
        let total_space = disk.total_space() / 1073741824;         // disk total space in GB
        let used_space = total_space - available_space;            // disk used space

        // push disk space data to result vector
        disks_space.push(
            DiskSpace {
                name: name.unwrap_or("Unknown").to_string(),
                available_space: available_space,
                total_space: total_space,
                used_space: used_space
            }
        )
    }

    return disks_space;
}

/// Returns all disks kind data.
pub fn get_disks_kind() -> Vec<DiskKind> {
    let disks = Disks::new_with_refreshed_list();

    let mut disks_kind: Vec<DiskKind> = Vec::new();
    for disk in &disks {
        let name = disk.name().to_str(); // disk name
        let kind = disk.kind().to_string();    // disk kind
        
        // push disk data to result vector
        disks_kind.push(
            DiskKind {
                name: name.unwrap_or("Unknown").to_string(),
                kind: kind
            }
        );
    }

    return disks_kind;
}
