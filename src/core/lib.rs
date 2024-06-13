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
    #[inline]
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
        outln_suc!(self.logger, "=======================NoitArchiver v{}=======================", env!("CARGO_PKG_VERSION"));

        let commands = self.com_analyzer.get_command_list();
        for (index, c) in commands.iter().enumerate() {
            out!(self.logger, "{}.{}({})\t{}", index+1, c.full_name, c.short_name, c.breif_info);
        }
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
        let para;
        match opt {
            Some(_) => para = opt.unwrap(),
            None => {
                out!(self.logger, "请输入存档名(直接换行则取消保存):");
                let arch_name = self.logger.io_getline().trim().to_string();
                if arch_name.is_empty() {
                    outln_log!(self.logger, "取消存档");
                    return Ok(());
                }
                out!(self.logger, "请输入存档备注(直接换行则不填):");
                let arch_note = self.logger.io_getline().trim().to_string();
                
                para = Save::new(&arch_name, &arch_note);
            }
        };

        let infos = self.file_manager.get_archive_infos();
        if (*infos).iter().any(|x| x.name == para.arch_name) {
            outln_warn!(self.logger, "已存在存档名为\"{}\",请重新命名", para.arch_name);
            return Ok(());
        }
        
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
        
        outln_suc!(self.logger, "保存成功");
        Ok(())
    }

    fn qsave(&mut self) -> Result<(), Error> {
        let time = Local::now();
        let date = [
            time.year() as usize,
            time.month() as usize,
            time.day() as usize,
        ];
        let time = [
            time.hour() as usize,
            time.minute() as usize,
            time.second() as usize,
        ];
        let info = file_manager::ArchiveInfo::new(
            &format!("qsave_{}", Self::generate_hashcode(&date, &time)),
            "",
            date,
            time,
        );
        self.file_manager.save(info).with_moreinfo("存档失败")?;
        outln_suc!(self.logger, "保存成功");
        Ok(())
    }

    fn generate_hashcode(date: &[usize;3], time: &[usize;3]) -> String {
        let mut transed: usize = 0;
        transed += date[1]-1;                       //month
        transed += (date[2]-1)*11;                  //day
        transed += time[0]*11*30;                   //hour
        transed += time[1]*11*30*23;                //minute
        transed += time[2]*11*30*23*59;             //second
        transed += (date[0]%100)*11*30*23*59*59;    //year

        let mut hashcode = String::new();
        while transed>0 {
            let a = (transed % 62) as u8;
            match a {
                0..=9 => hashcode = a.to_string() + &hashcode,
                10..=35 => hashcode = String::from((a-10+'a' as u8) as char) + &hashcode,
                36..=61 => hashcode = String::from((a-36+'A' as u8) as char) + &hashcode,
                _ => {}
            };
            transed = transed / 62;
        }
        hashcode
    }

    fn rsave(&mut self) -> Result<(), Error> {
        let infos = self.file_manager.get_archive_infos();
        let last = infos.len() - 1;
        out_warn!(self.logger, "此操作会覆盖存档 \"{}\" 请确认(y/n):", infos[last].to_string());
        if !self.logger.io_comfirm() {
            outln_log!(self.logger, "取消存档");
        }
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
        let para;
        match opt {
            Some(_) => para = opt.unwrap(),
            None => {
                out!(self.logger, "请输入存档编号(直接换行则取消读档):");
                if let Some(index) = self.logger.io_getint(){
                    para = Load::new((index - 1) as usize);
                } else {
                    outln_log!(self.logger, "取消读档");
                    return Ok(());
                }
            }
        };
        
        if para.index >= self.file_manager.get_archive_infolen() {
            outln_warn!(self.logger, "存档编号{}不存在", para.index);
            return Ok(());
        }
        
        out_warn!(self.logger, "此操作会用存档 \"{}\" 覆盖现有存档,请确认(y/n):",
            self.file_manager.get_archive_infos()[para.index].to_string());
        if !self.logger.io_comfirm() {
            outln_log!(self.logger, "取消存档");
        }

        self.file_manager
            .load(para.index)
            .with_moreinfo("读档失败")?;

        outln_suc!(self.logger, "读档成功");
        Ok(())
    }

    fn qload(&self) -> Result<(), Error> {
        let mut last = self.file_manager.get_archive_infolen();
        if last == 0 {
            outln_warn!(self.logger, "无存档可读取");
            return Ok(());
        }
        last -= 1;

        out_warn!(self.logger, "此操作会用存档 \"{}\" 覆盖现有存档,请确认(y/n):",
            self.file_manager.get_archive_infos()[last].to_string());
        if !self.logger.io_comfirm() {
            outln_log!(self.logger, "取消存档");
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
                "{:04}-{:02}-{:02}  {:02}:{:02}:{:02}",
                p.date[0], p.date[1], p.date[2], p.time[0], p.time[1], p.time[2]
            );
            outln_log!(
                self.logger,
                "[{}]  {}\t{}\t\t\t{}",
                index + 1,
                time_str,
                &p.name,
                &p.note
            );
        }
        Ok(())
    }

    fn modify_archive(&mut self, opt: Option<Modify>) -> Result<(), Error> {
        let para;
        match opt {
            Some(_) => para = opt.unwrap(),
            None => {
                out!(self.logger, "请输入存档编号(直接换行则取消修改):");
                if let Some(index) = self.logger.io_getint(){
                    out!(self.logger, "请输入存档名(直接换行则取消修改):");
                    let arch_name = self.logger.io_getline().trim().to_string();
                    if arch_name.is_empty() {
                        outln_log!(self.logger, "取消修改");
                        return Ok(());
                    }
                    out!(self.logger, "请输入存档备注(直接换行则不填):");
                    let arch_note = self.logger.io_getline().trim().to_string();
                    para = Modify::new((index - 1) as usize, &arch_name,&arch_note);
                } else {
                    outln_log!(self.logger, "取消修改");
                    return Ok(());
                }
            }
        };

        if para.index >= self.file_manager.get_archive_infolen() {
            outln_warn!(self.logger, "存档编号{}不存在", para.index);
            return Ok(());
        }
        
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

        outln_suc!(self.logger, "修改成功");
        Ok(())
    }

    fn del(&mut self, opt: Option<Del>) -> Result<(), Error> {
        let para;
        match opt {
            Some(_) => para = opt.unwrap(),
            None => {
                out!(self.logger, "请输入存档编号(直接换行则取消删除):");
                if let Some(index) = self.logger.io_getint(){
                    para = Del::new((index - 1) as usize);
                } else {
                    outln_log!(self.logger, "取消删除");
                    return Ok(());
                }
            }
        };
        
        if para.index >= self.file_manager.get_archive_infolen() {
            outln_warn!(self.logger, "存档编号{}不存在", para.index);
            return Ok(());
        }
        out_warn!(self.logger, "此操作会删除存档 \"{}\" 请确认(y/n):",
            self.file_manager.get_archive_infos()[para.index].to_string());
        if !self.logger.io_comfirm() {
            outln_log!(self.logger, "取消存档");
        }

        self.file_manager
            .del(para.index)
            .with_moreinfo("删除存档失败")?;

        outln_suc!(self.logger, "删除成功");
        Ok(())
    }

    fn qdel(&mut self) -> Result<(), Error> {
        let mut last = self.file_manager.get_archive_infolen();
        if last == 0 {
            outln_warn!(self.logger, "无存档");
            return Ok(());
        }
        
        last -= 1;
        out_warn!(self.logger, "此操作会用删除存档 \"{}\" 请确认(y/n):",
            self.file_manager.get_archive_infos()[last].to_string());
        if !self.logger.io_comfirm() {
            outln_log!(self.logger, "取消存档");
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
