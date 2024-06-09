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
    is_running: bool,
}

impl<'a, T: IOManager> Manager<'a, T> {
    pub fn new(logger: &'a T) -> Result<Self, Error> {
        let com_analyzer = com_analyzer::Analyzer::new();
        let file_manager = file_manager::FileManager::new(logger)?;
        Ok(Self {
            com_analyzer,
            file_manager,
            logger,
            is_running: true,
        })
    }
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn run_command(&mut self, command_input: &str) -> Result<(), Error> {
        let result = self.com_analyzer.analyze(command_input);
        let id;
        if let Err(e) = result {
            outln_warn!(self.logger, "{}", e);
            return Ok(());
        } else {
            id = result.unwrap();
        }
        // 获取infos长度
        let getlen = || self.file_manager.get_archive_infolen() as i32;

        match id {
            IdClear => self.clear(),
            IdHelp => self.help(),
            IdQuit => self.quit(),

            IdSave(opt) => self.save(opt),
            IdQsave => self.qsave(), //
            IdRsave(_) => self.rsave(),

            IdLoad(opt) => self.load(opt),
            IdQload => self.qload(), //
            IdLog => self.log(0..=getlen() - 1),
            IdSlog => self.log(getlen() - 7..=getlen() - 1),

            IdModarch(opt) => self.modify_archive(opt),
            IdDel(opt) => self.del(opt),
            IdQdel => self.qdel(), //
            IdUsage => self.usage(),
            IdFavor => self.favor(),
        }?;
        Ok(())
    }

    fn clear(&self) -> Result<(), Error> {
        self.logger.io_cls();
        // let commands = self.file_manager
        Ok(())
    }

    fn help(&self) -> Result<(), Error> {
        out!(self.logger, "help {}", 10);
        Ok(())
    }

    fn quit(&mut self) -> Result<(), Error> {
        self.is_running = false;
        Ok(())
    }

    fn save(&mut self, opt: Option<Save>) -> Result<(), Error> {
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
            self.file_manager.save(info).with_moreinfo("存档失败")?;
        } else {
        }
        outln_suc!(self.logger, "保存成功");
        Ok(())
    }

    fn qsave(&mut self) -> Result<(), Error> {
        let time = Local::now();
        let info = file_manager::ArchiveInfo::new(
            &format!("{}_quick_save", self.file_manager.get_archive_infolen()),
            "",
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
        self.file_manager.save(info).with_moreinfo("存档失败")?;
        outln_suc!(self.logger, "保存成功");
        Ok(())
    }

    fn rsave(&mut self) -> Result<(), Error> {
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
        self.file_manager
            .replace(last, new_info)
            .with_moreinfo("覆盖存储失败")?;
        outln_suc!(self.logger, "覆盖存储成功");
        Ok(())
    }

    fn load(&self, opt: Option<Load>) -> Result<(), Error> {
        if let Some(para) = opt {
            self.file_manager
                .load(para.index)
                .with_moreinfo("读档失败")?;
        } else {
        }
        outln_suc!(self.logger, "读档成功");
        Ok(())
    }

    fn qload(&self) -> Result<(), Error> {
        let last = self.file_manager.get_archive_infolen();
        if last == 0 {
            outln_warn!(self.logger, "存档编号不存在");
            return Ok(());
        }
        self.file_manager
            .load(last - 1)
            .with_moreinfo("快速读档失败")?;
        Ok(())
    }

    fn log(&self, range: RangeInclusive<i32>) -> Result<(), Error> {
        let infos = self.file_manager.get_archive_infos();
        if infos.len() == 0 {
            outln_warn!(self.logger, "无存档");
            return Ok(());
        }
        let start: usize = if *range.start() < 0 {
            0
        } else {
            *range.start() as usize
        };
        let end: usize = if *range.end() >= infos.len() as i32 {
            infos.len()
        } else {
            *range.end() as usize
        };

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
        Ok(())
    }

    fn modify_archive(&mut self, opt: Option<Modify>) -> Result<(), Error> {
        if let Some(para) = opt {
            let old_info = &self.file_manager.get_archive_infos()[para.index];
            let new_info = file_manager::ArchiveInfo::new(
                &para.info.arch_name,
                &para.info.arch_note,
                old_info.date,
                old_info.time,
            );
            self.file_manager
                .modify(para.index, new_info)
                .with_moreinfo("修改存档信息失败")?;
        } else {
        }
        outln_suc!(self.logger, "修改成功");
        Ok(())
    }

    fn del(&mut self, opt: Option<Del>) -> Result<(), Error> {
        if let Some(para) = opt {
            self.file_manager
                .del(para.index)
                .with_moreinfo("删除存档失败")?;
        } else {
        }
        outln_suc!(self.logger, "删除成功");
        Ok(())
    }

    fn qdel(&mut self) -> Result<(), Error> {
        let last = self.file_manager.get_archive_infolen();
        if last == 0 {
            outln_warn!(self.logger, "存档编号不存在");
            return Ok(());
        }
        self.file_manager
            .del(last - 1)
            .with_moreinfo("删除存档失败")?;
        outln_suc!(self.logger, "删除成功");
        Ok(())
    }

    fn usage(&self) -> Result<(), Error> {
        Ok(())
    }

    fn favor(&self) -> Result<(), Error> {
        Ok(())
    }
}
