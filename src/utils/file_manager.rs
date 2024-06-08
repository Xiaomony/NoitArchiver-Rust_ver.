use super::io_manager::{Error, IOManager};
use crate::{outln, outln_warn};

use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct ArchiveInfo {
    name: String,
    note: String,
    date: [usize; 3],
    time: [usize; 3],
}

struct JsonManager<'a, T: IOManager> {
    infos: Vec<ArchiveInfo>,
    path_to_json: PathBuf,
    logger: &'a T,
}

pub struct FileManager<'a, T: IOManager> {
    json_manager: JsonManager<'a, T>,
    path_to_noita_archive: PathBuf,
    path_to_archive_forlder: PathBuf,
    path_to_infos_json: PathBuf,

    logger: &'a T,
}

impl<'a, T: IOManager> JsonManager<'a, T> {
    fn new(path_to_json: PathBuf, logger: &'a T) -> Self {
        Self {
            infos: Vec::new(),
            path_to_json,
            logger,
        }
    }
    pub fn load_json(&mut self) -> Result<(), Error> {
        let f = fs::File::open(&self.path_to_json)?;
        let reader = BufReader::new(f);
        self.infos = serde_json::from_reader(reader)?;
        Ok(())
    }
    pub fn write_json(&self) -> Result<(), Error> {
        let f = fs::File::open(&self.path_to_json)?;
        serde_json::to_writer_pretty(f, &(self.infos))?;
        Ok(())
    }
    pub fn infos_push(&mut self, name: String, note: String, date: [usize; 3], time: [usize; 3]) {
        self.infos.push(ArchiveInfo {
            name,
            note,
            date,
            time,
        });
    }
    pub fn infos_del(&mut self, index: usize) {
        if index < self.infos.len() && index > 0 {
            self.infos.remove(index);
        } else {
            outln_warn!(self.logger, "存档编号不存在");
        }
    }
    pub fn infos_pop(&mut self) {
        if self.infos.len() > 0 {
            self.infos.pop();
        } else {
            outln_warn!(self.logger, "无存档可删除");
        }
    }
    pub fn infos_modify(&mut self, index: usize, new_name: String, new_note: String) {
        if index < self.infos.len() && index > 0 {
            self.infos[index].name = new_name;
            self.infos[index].note = new_note;
        } else {
            outln_warn!(self.logger, "存档编号不存在");
        }
    }
}

impl<'a, T: IOManager> FileManager<'a, T> {
    pub fn new(logger: &'a T) -> Self {
        let arch_path = Self::get_noita_arch_path();
        if let Err(err) = arch_path {
            outln!(*(logger), "{}", err);
            // wait until user press enter
            panic!();
        }
        let path_to_infos_json = PathBuf::from("./Archives/infos.json");
        let mut newone = Self {
            json_manager: JsonManager::new(path_to_infos_json.clone(), logger),
            path_to_noita_archive: PathBuf::from(arch_path.unwrap()),
            path_to_archive_forlder: PathBuf::from("./Archives"),
            path_to_infos_json,
            logger,
        };
        newone.init().unwrap();
        newone.json_manager.load_json().unwrap();
        newone
    }
    fn init(&self) -> Result<(), Error> {
        fs::create_dir_all(&self.path_to_archive_forlder)?;
        fs::File::create(&(self.path_to_infos_json))?;
        Ok(())
    }
    fn get_noita_arch_path() -> Result<String, Error> {
        Ok("./".to_string())
    }
}
