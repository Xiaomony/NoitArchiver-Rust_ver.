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
        addcom("help", "h", IdHelp(None), "帮助及注意事项\t",
            "查看帮助及注意事项, 请用 \"help/h [命令(全写或缩写)]\" 查看某个命令的说明及用法");
        addcom("quit", "q", IdQuit, "退出程序\n\n",
            "退出程序");

        addcom("save", "s", IdSave(None), "保存\t\t",
            "保存存档(存档名不能与已有存档重名)\n\t命令参数用法: save/s [存档名] [存档备注]\n\t存档备注可不填(默认为空)");
        addcom("qsave", "qs", IdQsave, "快速保存\t",
            "快速保存(存档名会以\"qsave_ + 生成的标识码\"的格式命名)");
        addcom("rsave", "rs", IdRsave(None), "覆盖式保存\n",
            "覆盖最新的一次存档(存档名和存档备注不变,更新存档日期)");

        addcom("load", "l", IdLoad(None), "读取存档\t",
            "读取存档\n\t命令参数用法: load/l [编号]");
        addcom("qload", "ql", IdQload, "快速读档\n",
            "读取最新的一次存档");

        addcom("log", "lg", IdLog, "查看存档信息\t",
            "查看存档信息");
        addcom("slog", "sl", IdSlog, "近七次存档信息\t",
            "仅查看最近七次的存档信息");
        addcom("modarch", "ma", IdModarch(None), "修改存档信息\n",
            "修改存档信息\n\t命令参数用法: modarch/ma [编号] [新存档名] [新备注]\n\t存档备注可不填(不填则保持旧的存档备注不变)");
        
        addcom("del", "d", IdDel(None), "删除指定存档\t",
            "删除存档\n\t命令参数用法: del/d [编号]");
        addcom("qdel", "qd", IdQdel, "删除最新存档\n",
            "删除最新的一次存档");

        addcom("favor", "f", IdFavor(None), "收藏存档\t",
            "收藏存档(使用favor命令收藏的存档不可进行任何修改，请先使用unfavor取消对其收藏才能修改)\n\t命令参数用法: favor/f [编号]");
        addcom("unfavor", "unf", IdUnfavor(None), "取消收藏\n\n",
            "取消收藏\n\t命令参数用法: unfavor/unf [编号]");
        
        addcom("usage", "use", IdUsage, "查看占用空间\n",
            "查看占用空间");

        Self {
            command_list: comlist,
        }
    }

    #[inline]
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

        let get_para_favor = |opt: &mut Option<_>| -> Result<(), Error> {
            if parts.len() <= 1 {
                *opt = None;
                return Ok(());
            }
            let index = parts[1].parse::<usize>().with_msg("命令格式错误")?;
            *opt = Some(Favor::new(index - 1));
            Ok(())
        };

        match id {
            IdHelp(ref mut opt) => {
                if parts.len() >= 2 {
                    if let Some(com) =
                        self.command_list.iter().find(|x| (**x).full_name==parts[1]||(**x).short_name==parts[1]) {
                            *opt = Some((com.full_name.clone(), com.short_name.clone(), com.detail.clone()));
                    } else {
                        return Err(Error::GeneralError("未找到命令".to_string()))
                    }
                }
                Ok(())
            }

            IdSave(ref mut opt) => {
                get_para_save(&mut *opt);
                Ok(())
            }
            IdLoad(ref mut opt) => get_para_load(&mut *opt),

            IdModarch(ref mut opt) => get_para_modify(&mut *opt),
            IdDel(ref mut opt) => get_para_del(&mut *opt),

            IdFavor(ref mut opt) | IdUnfavor(ref mut opt) =>
                get_para_favor(&mut *opt),
            _ => Ok(()),
        }?;
        Ok(id)
    }
}
