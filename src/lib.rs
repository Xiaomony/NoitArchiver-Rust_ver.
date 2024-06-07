mod utils;

use utils::io_manager::*;

use utils::com_analyzer;
use utils::commands::{CommandID::*, *};
use utils::file_manager;

pub struct Manager/*<T: OutLogger>*/ {
    com_analyzer: com_analyzer::Analyzer,
    file_manager: file_manager::FileManager,
    //logger: T
}

impl Manager {
    pub fn new() -> Self {
        let com_analyzer = com_analyzer::Analyzer::new();
        let file_manager = file_manager::FileManager::new();

        Self {
            com_analyzer,
            file_manager,
        }
    }

    pub fn run_command(&self, command_input: &str) {
        let id = self.com_analyzer.analyze(command_input);
        match id {
            IdErrCommand => {
                println!("error");
            }
            IdClear => Self::clear(),
            IdHelp => {
                println!("help");
            }
            IdQuit => {
                println!("quit");
            }
            IdSave(ref opt) => {
                println!("save {:?}", opt);
            }
            IdQsave(ref opt) => {
                println!("qsave {:?}", opt);
            }
            IdRsave(ref opt) => {
                println!("rsave {:?}", opt);
            }
            IdLoad(ref opt) => {
                println!("load {:?}", opt);
            }
            IdQload(ref opt) => {
                println!("qload {:?}", opt);
            }
            IdLog => {
                println!("log");
            }
            IdSlog => {
                println!("slog");
            }
            IdModarch(ref opt) => {
                println!("modarch {:?}", opt);
            }
            IdDel(ref opt) => {
                println!("del {:?}", opt);
            }
            IdQdel(ref opt) => {
                println!("qdel {:?}", opt);
            }
            IdUsage => {
                println!("usage");
            }
            IdFavor => {
                println!("favor");
            }
        }
    }

    fn clear() {}
}
