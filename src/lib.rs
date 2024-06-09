pub mod utils;

use utils::com_analyzer;
use utils::commands::{CommandID::*, *};
use utils::file_manager;
use utils::io_manager::*;

use chrono::prelude::*;
use std::ops::RangeInclusive;

pub struct Manager<'a, T: IOManager> {
    com_analyzer: com_analyzer::Analyzer,
    file_manager: file_manager::FileManager<'a, T>,
    logger: &'a T,
    is_running: bool
}

impl<'a, T: IOManager> Manager<'a, T> {
    pub fn new(logger: &'a T) -> Self {
        let com_analyzer = com_analyzer::Analyzer::new();
        let file_manager = file_manager::FileManager::new(logger);
        Self {
            com_analyzer,
            file_manager,
            logger,
            is_running: true,
        }
    }
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn run_command(&mut self, command_input: &str) {
        let id = self.com_analyzer.analyze(command_input);
        // 获取infos长度
        let getlen = || self.file_manager.get_archive_infolen() as i32;

        match id {
            IdErrCommand(err) => outln_warn!(self.logger, "{}", err),
            IdClear => self.clear(),
            IdHelp => self.help(),
            IdQuit => self.quit(),

            IdSave(opt) | IdQsave(opt) => self.save(opt),
            IdRsave(_) => self.rsave(),

            IdLoad(opt) => self.load(opt),
            IdQload(ref opt) => {
                println!("qload {:?}", opt);
            }
            IdLog => self.log(0..=getlen() - 1),
            IdSlog => self.log(getlen() - 7..=getlen() - 1),

            IdModarch(opt) => self.modify_archive(opt),
            IdDel(opt) => self.del(opt),
            IdQdel => {
                //
            }
            IdUsage => self.usage(),
            IdFavor => self.favor(),
        }
    }

    fn clear(&self) {
        self.logger.io_cls();
        // let commands = self.file_manager
    }

    fn help(&self) {
        out!(self.logger, "help {}", 10);
    }

    fn quit(&mut self) {
        self.is_running = false;
    }

    fn save(&mut self, opt: Option<Save>) {
        if let Some(para) = opt {
            let time = Local::now();
            let info = file_manager::ArchiveInfo::new(
                &para.arch_name,
                &para.arch_note,
                [
                    time.year() as usize,
                    time.month() as usize,
                    time.day() as usize,
                ],
                [
                    time.hour() as usize,
                    time.minute() as usize,
                    time.second() as usize,
                ],
            );
            self.file_manager.save(info).unwrap();
        } else {
        }
        outln_suc!(self.logger, "保存成功");
    }

    fn rsave(&mut self) {
        let infos = self.file_manager.get_archive_infos();
        let last = infos.len() - 1;
        let time = Local::now();
        let new_info = file_manager::ArchiveInfo::new(
            &infos[last].name,
            &infos[last].note,
            [
                time.year() as usize,
                time.month() as usize,
                time.day() as usize,
            ],
            [
                time.hour() as usize,
                time.minute() as usize,
                time.second() as usize,
            ],
        );
        self.file_manager.replace(last, new_info).unwrap();
    }

    fn load(&self, opt: Option<Load>) {
        if let Some(para) = opt {
            self.file_manager.load(para.index).unwrap();
        } else {
        }
    }

    fn log(&self, range: RangeInclusive<i32>) {
        let infos = self.file_manager.get_archive_infos();
        if infos.len() == 0 {
            outln_warn!(self.logger, "无存档");
            return;
        }
        let start:usize = if *range.start() < 0
                {0} else {*range.start() as usize};
        let end:usize = if *range.end() >= infos.len() as i32
                {infos.len()} else {*range.end() as usize};
        

        for (index, p) in infos[start..=end].iter().enumerate() {
            let time_str = format!(
                "{}-{}-{} {}:{}:{}",
                p.date[0], p.date[1], p.date[2], p.time[0], p.time[1], p.time[2]
            );
            outln_log!(
                self.logger,
                "[{}] {}\t{}\t{}",
                index + 1,
                time_str,
                &p.name,
                &p.note
            );
        }
    }

    fn modify_archive(&mut self, opt: Option<Modify>) {
        if let Some(para) = opt {
            let old_info = &self.file_manager.get_archive_infos()[para.index];
            let new_info = file_manager::ArchiveInfo::new(
                &para.info.arch_name,
                &para.info.arch_note,
                old_info.date,
                old_info.time,
            );
            self.file_manager.modify(para.index, new_info).unwrap();
        } else {
        }
    }

    fn del(&mut self, opt: Option<Del>) {
        if let Some(para) = opt {
            self.file_manager.del(para.index).unwrap();
        } else {
        }
    }

    fn usage(&self) {}

    fn favor(&self) {}
}
