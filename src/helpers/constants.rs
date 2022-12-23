// disk info table headers
pub const DISK_INFO_HEADERS: [&str; 6] = [
    "Filesystem",
    "1K-blocks",
    "Used",
    "Available",
    "Use Percentage",
    "Mounted on"];

// file info table headers
pub const FILE_INFO_HEADERS: [&str; 4] = [
    "Name",
    "Path",
    "Type",
    "Size(bytes)"
];

pub const DISK_INFO_HEADER_RELATIVE_SIZE: f32 = 1.0 / 8.0;
pub const INIT_WIN_WIDTH: f32 = 800.0;
pub const INIT_WIN_HEIGHT: f32 = 600.0;
pub const ERROR_STR: &str = "<error>";