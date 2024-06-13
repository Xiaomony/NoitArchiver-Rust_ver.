// use regex::Regex;

use super::commands::{CommandID::*, *};
use super::io_manager::{Error, ResultExt};

pub struct ComMap {
    pub full_name: String,
    pub short_name: String,
    id: CommandID,
    pub breif_info: String,
    pub detail: String
}
impl ComMap {
    fn new(full_name: &str, short_name: &str, id: CommandID, breif_info: &str, detail: &str) -> Self {
        Self {
            full_name: full_name.to_string(),
            short_name: short_name.to_string(),
            id,
            breif_info: breif_info.to_string(),
            detail: detail.to_string()
        }
    }
}

pub struct Analyzer {
    command_list: Vec<ComMap>,
}
impl Analyzer {
    pub fn new() -> Self {
        let mut comlist: Vec<ComMap> = Vec::new();
        let mut addcom = |full_name, short_name, id, breif_info, detail| {
            comlist.push(ComMap::new(full_name, short_name, id, breif_info, detail));
        };
        addcom("clear", "cls", IdClear, "清屏\t\t",
            "清除屏幕");
        addcom("help", "h", IdHelp, "帮助及注意事项\t",
            "帮助及注意事项");
        addcom("quit", "q", IdQuit, "退出程序\n\n",
            "退出程序");

        addcom("save", "s", IdSave(None), "保存\t\t",
            "保存存档");
        addcom("qsave", "qs", IdQsave, "快速保存\t",
            "");
        addcom("rsave", "rs", IdRsave(None), "覆盖式保存\n\n",
            "");

        addcom("load", "l", IdLoad(None), "读取存档\t",
            "");
        addcom("qload", "ql", IdQload, "快速读档\n\n",
            "");
        addcom("log", "lg", IdLog, "查看存档信息\t",
            "");
        addcom("slog", "sl", IdSlog, "近七次存档信息\n\n",
            "");

        addcom("modarch", "ma", IdModarch(None), "修改存档信息\t",
            "");
        addcom("del", "d", IdDel(None), "删除指定存档\t",
            "");
        addcom("qdel", "qd", IdQdel, "删除最新存档\n\n",
            "");

        addcom("usage", "u", IdUsage, "查看占用空间\t",
            "");
        addcom("favor", "f", IdFavor, "收藏存档\n\n",
            "");

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

        // 分离命令及其参数
        let mut result = Vec::new();
        let chars = trimmed_command.chars();
        let mut start = Some(0);
        let mut in_quote = false;
        let mut index: usize = 0;
        for c in chars.into_iter() {
            match c {
                '"' => {
                    if let Some(start_index) = start {
                        result.push(&trimmed_command[start_index..index]);
                        in_quote = false;
                        start = None;
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

    pub fn analyze(&self, command_input: &str) -> Result<CommandID, Error> {
        let parts: Vec<&str> = Self::preprocess_command(command_input);

        if parts.first() == None {
            return Err(Error::GeneralError("无命令输入".to_string()));
        }
        let head = parts.first().unwrap().to_string(); // head为所获得的命令

        // 闭包：寻找命令
        let find = |input: String| -> Result<CommandID, Error> {
            let id;
            for com in self.command_list.iter() {
                if input == com.short_name || input == com.full_name {
                    id = com.id.clone();
                    return Ok(id);
                }
            }
            Err(Error::GeneralError("未找到命令".to_string()))
        };
        let mut id = find(head)?;
        // 闭包：处理各种命令、装填参数
        let get_para_save = |opt: &mut Option<_>| match parts.len() {
            2 => *opt = Some(Save::new(parts[1], "")),
            len if len >= 3 => *opt = Some(Save::new(parts[1], parts[2])),
            _ => *opt = None,
        };

        let get_para_modify = |opt: &mut Option<_>| -> Result<(), Error> {
            if parts.len() <= 1 {
                *opt = None;
                return Ok(());
            }
            let index = parts[1].parse::<usize>().with_msg("命令格式错误")?;
            match parts.len() {
                3 => *opt = Some(Modify::new(index - 1, parts[2], "")),
                len if len >= 4 => *opt = Some(Modify::new(index - 1, parts[2], parts[3])),
                _ => *opt = None,
            }
            Ok(())
        };

        let get_para_load = |opt: &mut Option<_>| -> Result<(), Error> {
            if parts.len() <= 1 {
                *opt = None;
                return Ok(());
            }
            let index = parts[1].parse::<usize>().with_msg("命令格式错误")?;
            *opt = Some(Load::new(index - 1));
            Ok(())
        };

        let get_para_del = |opt: &mut Option<_>| -> Result<(), Error> {
            if parts.len() <= 1 {
                *opt = None;
                return Ok(());
            }
            let index = parts[1].parse::<usize>().with_msg("命令格式错误")?;
            *opt = Some(Del::new(index - 1));
            Ok(())
        };

        match id {
            IdSave(ref mut opt) => {
                get_para_save(&mut *opt);
                Ok(())
            }
            IdLoad(ref mut opt) => get_para_load(&mut *opt),

            IdModarch(ref mut opt) => get_para_modify(&mut *opt),
            IdDel(ref mut opt) => get_para_del(&mut *opt),
            _ => Ok(()),
        }?;
        Ok(id)
    }
}
