use std::collections::VecDeque;

pub struct Disk {
    pub file_system: String,
    pub one_k_blocks: String,
    pub used: String,
    pub available: String,
    pub use_percentage: String,
    pub mounted_on: String,
}

impl Disk {
    // scan for disks using df command
    pub(crate) fn from_df_line(line: &str) -> Disk {
        let part = line.split(" ").collect::<Vec<&str>>();
        let mut clean = VecDeque::new();
        for s in part {
            let clean_s = s.trim();
            if !clean_s.is_empty() {
                clean.push_back(clean_s);
            }
        }
        Disk {
            file_system: clean.pop_front().unwrap().to_string(),
            one_k_blocks: clean.pop_front().unwrap().to_string().to_string(),
            used: clean.pop_front().unwrap().to_string().to_string(),
            available: clean.pop_front().unwrap().to_string().to_string(),
            use_percentage: clean.pop_front().unwrap().replace("%", "").to_string(),
            mounted_on: clean.pop_front().unwrap().to_string().to_string(),
        }
    }
}