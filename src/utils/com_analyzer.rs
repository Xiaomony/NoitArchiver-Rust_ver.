// use regex::Regex;

use super::commands::{CommandID::*, *};

pub struct ComMap {
    full_name: String,
    short_name: String,
    id: CommandID,
}
impl ComMap {
    fn new(full_name: &str, short_name: &str, id: CommandID) -> Self {
        Self {
            full_name: full_name.to_string(),
            short_name: short_name.to_string(),
            id,
        }
    }
}
macro_rules! comm {
    ($full_name:expr, $short_name:expr, $id:expr) => {
        ComMap::new($full_name, $short_name, $id)
    };
}

pub struct Analyzer {
    command_list: Vec<ComMap>,
}
impl Analyzer {
    pub fn new() -> Self {
        let mut comlist: Vec<ComMap> = Vec::new();
        comlist.push(comm!("clear", "cls", IdClear));
        comlist.push(comm!("help", "h", IdHelp));
        comlist.push(comm!("quit", "q", IdQuit));

        comlist.push(comm!("save", "s", IdSave(None)));
        comlist.push(comm!("qsave", "qs", IdQsave(None)));
        comlist.push(comm!("rsave", "rs", IdRsave(None)));

        comlist.push(comm!("load", "l", IdLoad(None)));
        comlist.push(comm!("qload", "ql", IdQload(None)));
        comlist.push(comm!("log", "lg", IdLog));
        comlist.push(comm!("slog", "sl", IdSlog));

        comlist.push(comm!("modarch", "ma", IdModarch(None)));
        comlist.push(comm!("del", "d", IdDel(None)));
        comlist.push(comm!("qdel", "qd", IdQdel(None)));

        comlist.push(comm!("usage", "u", IdUsage));
        comlist.push(comm!("favor", "f", IdFavor));

        Self {
            command_list: comlist,
        }
    }

    pub fn get_command_list(&self) -> &Vec<ComMap> {
        &self.command_list
    }

    // 预先分隔、处理字符串
    fn preprocess_command<'a>(command: &'a str) -> Vec<&'a str> {
        // 掐头去尾
        let trimmed_command = command.trim();

        //trimmed_command.split(" ").collect()
        // 分离命令及其参数
        let mut result = Vec::new();
        let chars = trimmed_command.chars();
        let mut start = Some(0);
        let mut in_quote = false;
        let mut index:usize = 0;
        for c in chars.into_iter() {
            match c {
                '"' => {
                    if let Some(start_index) = start {
                        result.push(&trimmed_command[start_index..index]);
                        in_quote = false;
                    } else {
                        start = Some(index + 1);
                        in_quote = true;
                    }
                }
                ' ' => {
                    if !in_quote {
                        if let Some(start_index) = start {
                            result.push(&trimmed_command[start_index..index]);
                            start = None;
                        }
                    }
                }
                _ => {
                    if let None = start {
                        start = Some(index);
                    }
                }
            }
            index += c.len_utf8();
        }
        if let Some(start_index) = start {
            result.push(&trimmed_command[start_index..trimmed_command.len()])
        }
        result
    }

    pub fn analyze(&self, command_input: &str) -> CommandID {
        let parts: Vec<&str> = Self::preprocess_command(command_input);
        if parts.first() == None {
            return CommandID::IdErrCommand;
        }
        let head = parts.first().unwrap().to_string(); // head为所获得的命令

        // 闭包：寻找命令
        let find = |input: String| -> CommandID {
            let mut id = IdErrCommand;
            for com in self.command_list.iter() {
                if input == com.short_name || input == com.full_name {
                    id = com.id.clone();
                    break;
                }
            }
            id
        };
        let mut id = find(head);
        // 闭包：处理各种命令、装填参数
        let get_para_save = |opt: &mut Option<_>| match parts.len() {
            2 => *opt = Some(Save::new(parts[1], "")),
            len if len >= 3 => *opt = Some(Save::new(parts[1], parts[2])),
            _ => *opt = None,
        };

        let get_para_load = |opt: &mut Option<_>| {
            let index = parts[1].parse::<usize>();
            if let Err(_) = index {
                *opt = None
            } else {
                *opt = Some(Load::new(index.unwrap()));
            }
        };

        let get_para_del = |opt: &mut Option<_>| {
            let index = parts[1].parse::<usize>();
            if let Err(_) = index {
                *opt = None
            } else {
                *opt = Some(Del::new(index.unwrap()));
            }
        };

        match id {
            IdSave(ref mut opt) => get_para_save(&mut *opt),
            IdQsave(_) => {}
            IdRsave(_) => {}

            IdLoad(ref mut opt) => get_para_load(&mut *opt),
            IdQload(_) => {}

            IdModarch(ref mut opt) => get_para_save(&mut *opt),
            IdDel(ref mut opt) => get_para_del(&mut *opt),
            IdQdel(_) => {}
            _ => {}
        };
        id
    }
}
