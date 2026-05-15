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

pub fn get_all_disks_data() -> Vec<Disk> {
    let disks = Disks::new_with_refreshed_list();
    let mut disks_data: Vec<Disk> = Vec::new();

    for disk in &disks {
        let name = disk.name().to_str();
        let mount_point = disk.mount_point().to_str();
        let file_system = disk.file_system().to_str();
        let is_read_only = disk.is_read_only();
        let is_removable = disk.is_removable();
        let available_space = disk.available_space() / 1073741824;
        let total_space = disk.total_space() / 1073741824;
        let kind = disk.kind().to_string();
        
        // push disk data to result vector
        disks_data.push(
            Disk {
                name: name.unwrap_or("Unknown").to_string(),               // disk name
                mount_point: mount_point.unwrap_or("Unknown").to_string(), // disk mount point
                file_system: file_system.unwrap_or("Unknown").to_string(), // disk file system
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

pub fn get_disks_space_data() -> Vec<DiskSpace> {
    let disks = Disks::new_with_refreshed_list();
    let mut disks_space: Vec<DiskSpace> = Vec::new();

    for disk in &disks {
        let name = disk.name().to_str();
        let available_space = disk.available_space() / 1073741824;
        let total_space = disk.total_space() / 1073741824;

        // push disk space data to result vector
        disks_space.push(
            DiskSpace {
                name: name.unwrap_or("Unknown").to_string(),               // disk name
                available_space: available_space, // disk available space
                total_space: total_space,          // disk tottal space
                used_space: total_space - available_space
            }
        )
    }

    return disks_space;
}

pub fn get_disks_kind() -> Vec<DiskKind> {
    let disks = Disks::new_with_refreshed_list();
    let mut disks_kind: Vec<DiskKind> = Vec::new();

    for disk in &disks {
        let name = disk.name().to_str();
        let kind = disk.kind().to_string();
        
        // push disk data to result vector
        disks_kind.push(
            DiskKind {
                name: name.unwrap_or("Unknown").to_string(), // disk name
                kind: kind                                           // disk kind
            }
        );
    }

    return disks_kind;
}