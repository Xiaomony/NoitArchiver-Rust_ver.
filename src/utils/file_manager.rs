struct ArchiveInfo {
    name: String,
    note: String,
    date: [usize; 3],
    time: [usize; 3],
}

struct JsonManager {
    infos: Vec<ArchiveInfo>,
    
}

pub struct FileManager {
    json_manager: JsonManager,
}

impl JsonManager {
    fn new() -> Self {
        Self { infos: Vec::new() }
    }
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            json_manager: JsonManager::new(),
        }
    }
}
