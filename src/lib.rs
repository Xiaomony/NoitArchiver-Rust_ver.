pub mod utils;

use utils::com_analyzer;
use utils::commands::{CommandID::*, *};
use utils::file_manager;
use utils::io_manager::*;

use std::ops::RangeInclusive;

pub struct Manager<T: IOManager> {
    com_analyzer: com_analyzer::Analyzer,
    file_manager: file_manager::FileManager,
    logger: T,
    //exit_callback: fn ()
}

impl<T: IOManager> Manager<T> {
    pub fn new(logger: T) -> Self {
        let com_analyzer = com_analyzer::Analyzer::new();
        let file_manager = file_manager::FileManager::new();

        Self {
            com_analyzer,
            file_manager,
            logger,
        }
    }

    pub fn run_command(&self, command_input: &str) {
        let id = self.com_analyzer.analyze(command_input);
        match id {
            IdErrCommand => outln_warn!(self.logger, "未找到命令 {}", 10),
            IdClear => self.clear(),
            IdHelp => self.help(),
            IdQuit => self.quit(),

            IdSave(opt) | IdQsave(opt) => self.save(opt),
            IdRsave(_) => self.rsave(),

            IdLoad(opt) => self.load(opt),
            IdQload(ref opt) => {
                println!("qload {:?}", opt);
            }
            IdLog => self.log(0..=1),
            IdSlog => self.log(0..=1),

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
    }

    fn help(&self) {
        out!(self.logger, "help {}", 10);
    }

    fn quit(&self) {
        out_log!(self.logger, "退出程序");
    }

    fn save(&self, opt: Option<Save>) {
        if let Some(para) = opt {
        } else {
        }
        out_suc!(self.logger, "保存成功");
    }

    fn rsave(&self) {}

    fn load(&self, opt: Option<Load>) {
        if let Some(para) = opt {
        } else {
        }
    }

    fn log(&self, range: RangeInclusive<usize>) {}

    fn modify_archive(&self, opt: Option<Save>) {
        if let Some(para) = opt {
        } else {
        }
    }

    fn del(&self, opt: Option<Del>) {
        if let Some(para) = opt {
        } else {
        }
    }

    fn usage(&self) {}

    fn favor(&self) {}
}
