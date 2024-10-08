pub mod utils;

use utils::com_analyzer;
use utils::commands::{CommandID::*, *};
use utils::file_manager;
use utils::io_manager::*;

use chrono::prelude::*;
use std::ops::RangeInclusive;
use std::process::Command;

pub struct Manager<'a, T: IOManager> {
    com_analyzer: com_analyzer::Analyzer,
    pub file_manager: file_manager::FileManager<'a, T>,
    logger: &'a T,
    is_running: bool,
}

impl<'a, T: IOManager> Manager<'a, T> {
    pub const HELP_MSG:&'static str = include_str!("./help_str.txt");

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
            IdStartGame => { self.startgame() },
            IdClear => self.clear(),
            IdHelp(ref opt) => self.help(opt),
            IdQuit => self.quit(),

            IdSave(opt) => self.save(opt),
            IdQsave => self.qsave(),
            IdRsave(_) => self.rsave(),

            IdLoad(opt) => self.load(opt),
            IdQload => self.qload(),
            IdLog => self.log(0..=getlen() - 1),
            IdSlog => self.log(getlen() - 7..=getlen() - 1),

            IdModarch(opt) => self.modify_archive(opt),
            IdDel(opt) => self.del(opt),
            IdQdel => self.qdel(),

            IdFavor(opt) => self.favor(opt, true),
            IdUnfavor(opt) => self.favor(opt, false),
            IdUsage => self.usage(),
        }?;
        Ok(())
    }

    fn startgame(&mut self) -> Result<(), Error> {
        self.file_manager.load_noita_path()?;
        let noita_path_opt = self.file_manager.get_noita_path();
        if noita_path_opt == None {
            outln_err!(self.logger, "请在Archives文件夹下的noita_path.txt中输入正确的Noita.exe的路径");
        } else {
            let noita_path = noita_path_opt.unwrap();
            outln_warn!(self.logger, "如果在Steam未启动时使用此命令，会无法加载Steam中的模组和存档（若Steam已启动，请忽略此提示）");
            let dir_path = noita_path.parent().unwrap();
            Command::new("cmd")
                .arg("/C")
                .arg(&noita_path)
                .current_dir(dir_path)
                .spawn()
                .expect("启动noita失败");
        }
        Ok(())
    }

    fn clear(&self) -> Result<(), Error> {
        self.logger.io_cls();
        outln_suc!(self.logger, "==================================NoitArchiver v{}==================================", env!("CARGO_PKG_VERSION"));
        
        let commands = self.com_analyzer.get_command_list();
        for (index, c) in commands.iter().enumerate() {
            out!(self.logger, "{}.{}({})\t{}", index+1, c.full_name, c.short_name, c.breif_info);
            }

        outln_suc!(self.logger, "=======================================================================================");
        Ok(())
    }

    fn help(&self, opt: &Option<(String, String, String)>) -> Result<(), Error> {
        match opt {
            None => {
                outln_log!(self.logger, "{}", Self::HELP_MSG);
                outln_suc!(self.logger, "本程序相关说明及介绍视频:https://www.bilibili.com/video/BV1UZbkeNEoc/?vd_source=b94494fff1b1147eb0072109b3ee55bc");
            }
            Some((full_name, short_name, detail)) => {
                outln_log!(self.logger, "\t[{}] ({})\n\t{}", full_name, short_name, detail);
            }
        }
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
        let mut last = self.file_manager.get_archive_infolen();
        if last == 0 {
            outln_warn!(self.logger, "无存档");
            return Ok(());
        }
        last -= 1;
        let infos = self.file_manager.get_archive_infos();
        if infos[last].get_is_favored() {
            outln_warn!(self.logger, "存档被收藏,无法修改");
            return Ok(());
        }

        //out_warn!(self.logger, "此操作会覆盖存档 \"{}\" 请确认", infos[last].to_string());
        if !get_confirm!(self.logger, "此操作会覆盖存档 \"{}\" 请确认", infos[last].to_string()) {
            outln_log!(self.logger, "取消存档");
            return Ok(());
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
        
        //out_warn!(self.logger,);
        if !get_confirm!(self.logger,  "此操作会用存档 \"{}\" 覆盖现有存档,请确认",
            self.file_manager.get_archive_infos()[para.index].to_string()) {
            outln_log!(self.logger, "取消读档");
            return Ok(());
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

        //out_warn!(self.logger, );
        if !get_confirm!(self.logger, "此操作会用存档 \"{}\" 覆盖现有存档,请确认",
            self.file_manager.get_archive_infos()[last].to_string()) {
            outln_log!(self.logger, "取消读档");
            return Ok(());
        }

        self.file_manager
            .load(last)
            .with_moreinfo("快速读档失败")?;

        outln_suc!(self.logger, "读档成功");
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
        outln_log!(self.logger, "收藏的存档使用*标注");
        for (index, p) in infos[start..=end].iter().enumerate() {
            let time_str = format!(
                "{:04}-{:02}-{:02}  {:02}:{:02}:{:02}",
                p.date[0], p.date[1], p.date[2], p.time[0], p.time[1], p.time[2]
            );
            if p.get_is_favored() {
                outln_suc!(
                    self.logger,
                    "[{}]* {}\t{}\t\t{}",
                    start + index + 1,
                    time_str,
                    &p.name,
                    &p.note
                    );
            } else {
                outln_log!(
                    self.logger,
                    "[{}]  {}\t{}\t\t{}",
                    start + index + 1,
                    time_str,
                    &p.name,
                    &p.note
                    );
            }
        }
        Ok(())
    }

    fn modify_archive(&mut self, opt: Option<Modify>) -> Result<(), Error> {
        let mut para;
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
                    out!(self.logger, "请输入存档备注(直接换行则保持不变):");
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
        if old_info.get_is_favored() {
            outln_warn!(self.logger, "存档被收藏,无法修改");
            return Ok(());
        }

        if para.info.arch_note.is_empty() {
            para.info.arch_note = old_info.note.clone();
        }
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

    fn del(&mut self,opt: Option<Del>) -> Result<(), Error> {
        let mut paras;
        let mut comfirm = false;
        match opt {
            Some(_) => paras = opt.unwrap(),
            None => {
                out!(self.logger, "请输入存档编号(直接换行则取消删除):");
                if let Some(index) = self.logger.io_getint(){
                    paras = Del::new_slices(&[(index - 1) as usize]);
                } else {
                    outln_log!(self.logger, "取消删除");
                    return Ok(());
                }
                comfirm = true;
            }
        };
        paras.indexs.sort_by(|a, b| b.cmp(a));
        paras.indexs.dedup();
        if !comfirm {
            let index_add1:Vec<usize> = paras.indexs.iter().map(|x| x+1).collect();
            //out_warn!(self.logger, "此操作会删除存档 \"{:?}\" 请确认",index_add1);
            if !get_confirm!(self.logger, "此操作会删除存档 \"{:?}\" 请确认",index_add1) {
                outln_log!(self.logger, "取消删除");
                return Ok(());
            }
        }
        for index in paras.indexs {
            if index >= self.file_manager.get_archive_infolen() {
                outln_warn!(self.logger, "存档编号[{}]不存在", index+1);
                return Ok(());
            }
            if self.file_manager.get_archive_infos()[index].get_is_favored() {
                outln_warn!(self.logger, "存档[{}]被收藏,无法操作", index+1);
                continue;
            }
    
            if comfirm {    
                //out_warn!(self.logger,);
                if !get_confirm!(self.logger, "此操作会删除存档 \"{}\" 请确认",
                        self.file_manager.get_archive_infos()[index].to_string()) {
                    outln_log!(self.logger, "取消删除");
                    return Ok(());
                }
            }

            let del_info = self.file_manager.get_archive_infos()[index].to_string();
            self.file_manager
                .del(index)
                .with_moreinfo("删除存档失败")?;
    
            outln_suc!(self.logger, "成功删除存档({}) \"{}\"", index + 1, del_info);
        }
        Ok(())
    }

    fn qdel(&mut self) -> Result<(), Error> {
        let mut last = self.file_manager.get_archive_infolen();
        if last == 0 {
            outln_warn!(self.logger, "无存档");
            return Ok(());
        }
        
        last -= 1;
        if self.file_manager.get_archive_infos()[last].get_is_favored() {
            outln_warn!(self.logger, "存档被收藏,无法修改");
            return Ok(());
        }
        //out_warn!(self.logger,);
        if !get_confirm!(self.logger, "此操作会用删除存档 \"{}\" 请确认",
                self.file_manager.get_archive_infos()[last].to_string()) {
            outln_log!(self.logger, "取消删除");
            return Ok(());
        }

        self.file_manager
            .del(last)
            .with_moreinfo("删除存档失败")?;
        outln_suc!(self.logger, "删除成功");
        Ok(())
    }

    fn usage(&self) -> Result<(), Error> {
        let usage = self.file_manager.get_usage()?;
        outln_log!(self.logger, "占用大小: {:.2} MB", usage);
        Ok(())
    }

    fn favor(&mut self, opt: Option<Favor>, state: bool) -> Result<(), Error> {
        let para;
        match opt {
            Some(_) => para = opt.unwrap(),
            None => {
                out!(self.logger, "请输入存档编号(直接换行则取消删除):");
                if let Some(index) = self.logger.io_getint(){
                    para = Favor::new((index - 1) as usize);
                } else {
                    outln_log!(self.logger, "取消");
                    return Ok(());
                }
            }
        };
        
        if para.index >= self.file_manager.get_archive_infolen() {
            outln_warn!(self.logger, "存档编号{}不存在", para.index);
            return Ok(());
        }
        self.file_manager.get_archive_mutinfos()[para.index].set_favored(state);
        self.file_manager.save_json()?;
        if state {
            outln_suc!(self.logger, "收藏成功");
        } else {
            outln_suc!(self.logger, "取消收藏成功");
        }
        Ok(())
    }
}
