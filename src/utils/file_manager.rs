use super::io_manager::{Error, IOManager};
use crate::out;

//use std::fs::File;
use std::path::PathBuf;

struct ArchiveInfo {
    name: String,
    note: String,
    date: [usize; 3],
    time: [usize; 3],
}

struct JsonManager {
    infos: Vec<ArchiveInfo>,
}

pub struct FileManager<'a, T: IOManager> {
    json_manager: JsonManager,
    path_to_noita_archive: PathBuf,
    path_to_archive_forlder: PathBuf,
    path_to_infos_json: PathBuf,

    logger: &'a T,
}

impl JsonManager {
    fn new() -> Self {
        Self { infos: Vec::new() }
    }
}

impl<'a, T: IOManager> FileManager<'a, T> {
    pub fn new(logger: &'a T) -> Self {
        let arch_path = Self::get_noita_arch_path();
        if let Err(err) = arch_path {
            out!(*(logger), "{}", err);
            // wait until user press enter
            panic!();
        }
        Self {
            json_manager: JsonManager::new(),
            path_to_noita_archive: PathBuf::from(arch_path.unwrap()),
            path_to_archive_forlder: PathBuf::from("./Archives"),
            path_to_infos_json: PathBuf::from("./Archives/infos.json"),
            logger,
        }
    }
    fn get_noita_arch_path() -> Result<String, Error> {
        Ok("./".to_string())
    }
}
