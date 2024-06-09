use super::io_manager::{Error, IOManager};
use crate::{outln, outln_err, outln_warn};

use serde::{Deserialize, Serialize};
use serde_json;
//use serde_json::ser::PrettyFormatter;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct ArchiveInfo {
    pub name: String,
    pub note: String,
    pub date: [usize; 3],
    pub time: [usize; 3],
}

impl ArchiveInfo {
    pub fn new(name: &str, note: &str, date: [usize; 3], time: [usize; 3]) -> Self {
        Self {
            name: name.to_string(),
            note: note.to_string(),
            date,
            time,
        }
    }
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

    pub fn get_archive_infos(&self) -> &Vec<ArchiveInfo> {
        &self.infos
    }

    pub fn load_json(&mut self) -> Result<(), Error> {
        let f = File::open(&self.path_to_json)?;
        let reader = BufReader::new(f);
        if let Ok(info) = serde_json::from_reader(reader) {
            self.infos = info;
        } else {
            outln_err!(self.logger, "Json文件格式错误,尝试手动修复./Archives/infos.json或删除Archive文件夹(删除Archive文件夹会导致存档的丢失！！！)");
            panic!()
        }
        Ok(())
    }
    pub fn write_json(&self) -> Result<(), Error> {
        let f = File::create(&self.path_to_json)?;
        // ***********here needs to be pretty formatter***********
        serde_json::to_writer_pretty(f, &(self.infos))?;
        Ok(())
    }
    pub fn infos_push(&mut self, info: ArchiveInfo) {
        self.infos.push(info);
    }
    pub fn infos_del(&mut self, index: usize) {
        if index < self.infos.len()
        /*&& index >= 0*/
        {
            self.infos.remove(index);
        } else {
            outln_warn!(self.logger, "存档编号不存在");
        }
    }
    pub fn _infos_pop(&mut self) {
        if self.infos.len() > 0 {
            self.infos.pop();
        } else {
            outln_warn!(self.logger, "无存档可删除");
        }
    }
    pub fn infos_modify(&mut self, index: usize, new_info: ArchiveInfo) {
        if index < self.infos.len() && index > 0 {
            self.infos[index] = new_info
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
        if !self.path_to_infos_json.exists() {
            let mut f = File::create(&(self.path_to_infos_json))?;
            f.write_all(b"[\n]")?;
        }
        Ok(())
    }
    fn get_noita_arch_path() -> Result<String, Error> {
        Ok("/home/runner/NoitaArchiveManagerRust-ver/NoitaArch".to_string())
    }

    pub fn get_archive_infos(&self) -> &Vec<ArchiveInfo> {
        self.json_manager.get_archive_infos()
    }
    pub fn get_archive_infolen(&self) -> usize {
        self.get_archive_infos().len()
    }
    pub fn save(&mut self, info: ArchiveInfo) -> Result<(), Error> {
        let dst_path = self.path_to_archive_forlder.join(&info.name);
        Self::copy_file(&self.path_to_noita_archive, &dst_path)?;
        self.json_manager.infos_push(info);
        self.json_manager.write_json()?;
        Ok(())
    }
    pub fn del(&mut self, index: usize) -> Result<(), Error> {
        let arch_info = &(self.json_manager.get_archive_infos()[index]);
        fs::remove_dir_all(self.path_to_archive_forlder.join(&arch_info.name))?;
        self.json_manager.infos_del(index);
        self.json_manager.write_json()?;
        Ok(())
    }
    pub fn replace(&mut self, index: usize, new_info: ArchiveInfo) -> Result<(), Error> {
        let old_info = &self.json_manager.get_archive_infos()[index];
        fs::remove_dir_all(self.path_to_archive_forlder.join(&(old_info.name)))?;
        Self::copy_file(
            &self.path_to_noita_archive,
            &self.path_to_archive_forlder.join(&new_info.name),
        )?;
        self.json_manager.infos_modify(index, new_info);
        self.json_manager.write_json()?;
        Ok(())
    }
    pub fn modify(&mut self, index: usize, new_info: ArchiveInfo) -> Result<(), Error> {
        let old_path = self
            .path_to_archive_forlder
            .join(&self.get_archive_infos()[index].name);
        let new_path = self.path_to_archive_forlder.join(&new_info.name);
        fs::rename(&old_path, &new_path)?;
        self.json_manager.infos_modify(index, new_info);
        self.json_manager.write_json()?;
        Ok(())
    }
    pub fn load(&self, index: usize) -> Result<(), Error> {
        let arch_name = &self.get_archive_infos()[index].name;
        let src = self.path_to_archive_forlder.join(arch_name);
        //outln_warn!(self.logger, "{:?}", src);
        Self::copy_file(&src, &self.path_to_noita_archive)?;
        Ok(())
    }

    fn copy_file(src: &Path, dst: &Path) -> Result<(), Error> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                Self::copy_file(&path, &dst.join(path.file_name().unwrap()))?;
            } else {
                fs::copy(&path, dst.join(path.file_name().unwrap()))?;
            }
        }
        Ok(())
    }
}
