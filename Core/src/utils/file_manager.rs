use super::io_manager::{Error, IOManager, ResultExt};
use crate::outln_warn;

use serde::{Deserialize, Serialize};
use serde_json;
//use serde_json::ser::Formatter;
//use serde_json::Serializer;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
//use std::io::BufWriter;
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ArchiveInfo {
    pub name: String,
    pub note: String,
    pub date: [usize; 3],
    pub time: [usize; 3],
    is_favored: bool,
}

// struct OneLineArrayFormatter/*<'a, W: Write>*/ {
//     //inner: &'a mut W,
// }

// impl Formatter for OneLineArrayFormatter {
//     fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> std::io::Result<()>
//     where
//         W: ?Sized + std::io::Write,
//     {
//         writer.write_all(if first { b"" } else { b"," })
//         //indent(writer, self.current_indent, self.indent)
//     }
// }

impl ArchiveInfo {
    pub fn new(name: &str, note: &str, date: [usize; 3], time: [usize; 3]) -> Self {
        Self {
            name: name.to_string(),
            note: note.to_string(),
            date,
            time,
            is_favored: false,
        }
    }
    #[inline]
    pub fn to_string(&self) -> String {
        format!("[{}-{}-{} {}:{}:{}]  {}",
            self.date[0], self.date[1], self.date[2],
            self.time[0], self.time[1], self.time[2],
            self.name)
    }
    #[inline]
    pub fn get_is_favored(&self) -> bool {
        self.is_favored
    }
    #[inline]
    pub fn set_favored(&mut self, state: bool) {
        self.is_favored = state;
    }
}

struct JsonManager<'a, T: IOManager> {
    infos: Vec<ArchiveInfo>,
    path_to_json: PathBuf,
    _logger: &'a T,
}

pub struct FileManager<'a, T: IOManager> {
    json_manager: JsonManager<'a, T>,
    path_to_noita_archive: PathBuf,
    path_to_archive_forlder: PathBuf,
    path_to_infos_json: PathBuf,

    logger: &'a T,
}

impl<'a, T: IOManager> JsonManager<'a, T> {
    fn new(path_to_json: PathBuf, _logger: &'a T) -> Self {
        Self {
            infos: Vec::new(),
            path_to_json,
            _logger,
        }
    }

    #[inline]
    pub fn get_archive_infos(&self) -> &Vec<ArchiveInfo> {
        &self.infos
    }
    #[inline]
    pub fn get_archive_mutinfos(&mut self) -> &mut Vec<ArchiveInfo> {
        &mut self.infos
    }

    pub fn load_json(&mut self) -> Result<(), Error> {
        let f = File::open(&self.path_to_json)
            .with_moreinfo("打开备份信息文件(./Archives/infos.json)失败")?;
        let reader = BufReader::new(f);
        self.infos = serde_json::from_reader(reader).with_msg(
            "读取存档信息文件失败\n可能为Json文件格式错误,尝试手动修复./Archives/infos.json(详见本程序gitHub主页的说明文档: https://github.com/Xiaomony/NoitArchiver-Rust_ver.)
\n或删除Archive文件夹(删除Archive文件夹会导致存档的丢失！！！)"
        )?;
        Ok(())
    }
    pub fn write_json(&self) -> Result<(), Error> {
        let f = File::create(&self.path_to_json).with_moreinfo("打开备份json文件失败")?;
        // ***********here needs to be pretty formatter***********
        serde_json::to_writer_pretty(f, &(self.infos)).with_msg("写入备份json文件失败")?;
        // let writer = BufWriter::new(f);
        // let mut ser = Serializer::with_formatter(writer, OneLineArrayFormatter{});
        // self.infos.serialize(&mut ser).with_msg("写入备份json文件失败")?;
        Ok(())
    }
    #[inline]
    pub fn infos_push(&mut self, info: ArchiveInfo) {
        self.infos.push(info);
    }
    #[inline]
    pub fn infos_del(&mut self, index: usize) {
        self.infos.remove(index);
    }
    #[inline]
    pub fn infos_modify(&mut self, index: usize, new_info: ArchiveInfo) {
        self.infos[index] = new_info;
    }
}

impl<'a, T: IOManager> FileManager<'a, T> {
    pub fn new(logger: &'a T) -> Result<Self, Error> {
        let arch_path = Self::get_noita_arch_path()
            .with_msg("获取Noita存档路径失败,请检查Noita是否安装")?;
        let path_to_infos_json = PathBuf::from("./Archives/infos.json");
        let mut newone = Self {
            json_manager: JsonManager::new(path_to_infos_json.clone(), logger),
            path_to_noita_archive: arch_path,
            path_to_archive_forlder: PathBuf::from("./Archives"),
            path_to_infos_json,
            logger,
        };
        newone.init()?;
        newone.json_manager.load_json()?;
        Ok(newone)
    }
    fn init(&self) -> Result<(), Error> {
        fs::create_dir_all(&self.path_to_archive_forlder).with_moreinfo("创建备份文件夹失败")?;
        if !self.path_to_infos_json.exists() {
            let mut f =
                File::create(&(self.path_to_infos_json)).with_moreinfo("创建备份json文件失败")?;
            f.write_all(b"[\n]").with_msg("初始化备份json文件失败")?;
        }
        Ok(())
    }
    fn get_noita_arch_path() -> Result<PathBuf, Error> {
        if let Some(appdata) = dirs::data_local_dir() {
            let locallow_path = appdata.parent().unwrap().join("LocalLow/Nolla_Games_Noita/save00");
            Ok(locallow_path)
        } else {
            Err(Error::GeneralError("".to_string()))
        }
    }

    #[inline]
    pub fn get_archive_infos(&self) -> &Vec<ArchiveInfo> {
        self.json_manager.get_archive_infos()
    }
    #[inline]
    pub fn get_archive_mutinfos(&mut self) -> &mut Vec<ArchiveInfo> {
        self.json_manager.get_archive_mutinfos()
    }
    #[inline]
    pub fn get_archive_infolen(&self) -> usize {
        self.get_archive_infos().len()
    }
    pub fn save(&mut self, info: ArchiveInfo) -> Result<(), Error> {
        let dst_path = self.path_to_archive_forlder.join(&info.name);
        Self::copy_file(&self.path_to_noita_archive, &dst_path)
            .with_moreinfo("复制存档失败(请确保Noita完全关闭且程序拥有相关权限)")?;
        self.json_manager.infos_push(info);
        self.json_manager.write_json()?;
        Ok(())
    }
    pub fn del(&mut self, index: usize) -> Result<(), Error> {
        if index >= self.get_archive_infolen() {
            outln_warn!(self.logger, "存档编号不存在");
            return Ok(());
        }
        let arch_info = &(self.json_manager.get_archive_infos()[index]);
        fs::remove_dir_all(self.path_to_archive_forlder.join(&arch_info.name))
            .with_moreinfo("删除存档文件夹失败,请尝试手动删除")?;
        self.json_manager.infos_del(index);
        self.json_manager.write_json()?;
        Ok(())
    }
    pub fn replace(&mut self, index: usize, new_info: ArchiveInfo) -> Result<(), Error> {
        if index >= self.get_archive_infolen() {
            outln_warn!(self.logger, "存档编号不存在");
            return Ok(());
        }

        let old_info = &self.json_manager.get_archive_infos()[index];
        fs::remove_dir_all(self.path_to_archive_forlder.join(&(old_info.name)))
            .with_moreinfo("删除旧的存档文件夹失败,请尝试手动替换存档")?;
        Self::copy_file(
            &self.path_to_noita_archive,
            &self.path_to_archive_forlder.join(&new_info.name),
        )
        .with_moreinfo("复制新存档失败,尝试手动替换存档")?;
        self.json_manager.infos_modify(index, new_info);
        self.json_manager.write_json()?;
        Ok(())
    }
    pub fn modify(&mut self, index: usize, new_info: ArchiveInfo) -> Result<(), Error> {
        if index >= self.get_archive_infolen() {
            outln_warn!(self.logger, "存档编号不存在");
            return Ok(());
        }

        let old_path = self
            .path_to_archive_forlder
            .join(&self.get_archive_infos()[index].name);
        let new_path = self.path_to_archive_forlder.join(&new_info.name);
        fs::rename(&old_path, &new_path).with_moreinfo("重命名存档文件夹失败,尝试手动操作")?;
        self.json_manager.infos_modify(index, new_info);
        self.json_manager.write_json()?;
        Ok(())
    }
    pub fn load(&self, index: usize) -> Result<(), Error> {
        if index >= self.get_archive_infolen() {
            outln_warn!(self.logger, "存档编号不存在");
            return Ok(());
        }

        let arch_name = &self.get_archive_infos()[index].name;
        let src = self.path_to_archive_forlder.join(arch_name);
        fs::remove_dir_all(&self.path_to_noita_archive).with_moreinfo("删除旧文件夹失败,请尝试手动删除")?;
        Self::copy_file(&src, &self.path_to_noita_archive).with_moreinfo("复制存档文件夹失败")?;
        Ok(())
    }

    fn copy_file(src: &Path, dst: &Path) -> Result<(), Error> {
        if !src.exists() {
            return Err(Error::GeneralError(format!("未找到目录:{}",src.to_str().unwrap())));
        }
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
    #[inline]
    pub fn save_json(&self) -> Result<(), Error> {
        self.json_manager.write_json()
    }

    pub fn get_usage(&self) -> Result<f64, Error> {
        let size = Self::caculate_usage(&self.path_to_archive_forlder)?;
        Ok(size)
    }
    fn caculate_usage(path: &Path) -> Result<f64, Error> {
        let mut total_size = 0 as f64;
    
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
    
            if metadata.is_dir() {
                total_size += Self::caculate_usage(&entry.path())?;
            } else {
                total_size += metadata.len() as f64 / 1_048_576.0;
            }
        }
    
        Ok(total_size)
    }    
}
