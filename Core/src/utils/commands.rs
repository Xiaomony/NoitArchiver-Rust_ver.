//use super::io_manager::Error;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum CommandID {
    IdClear,
    IdHelp(Option<(String, String, String)>),
    IdQuit,

    IdSave(Option<Save>),
    IdQsave,
    IdRsave(Option<Save>),

    IdLoad(Option<Load>),
    IdQload,
    IdLog,
    IdSlog,

    IdModarch(Option<Modify>),
    IdDel(Option<Del>),
    IdQdel,

    IdFavor(Option<Favor>),
    IdUnfavor(Option<Favor>),
    IdUsage,
}

#[derive(Clone, Debug, Serialize)]
pub struct Save {
    pub arch_name: String,
    pub arch_note: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Modify {
    pub index: usize,
    pub info: Save,
}

#[derive(Clone, Debug, Serialize)]
pub struct Load {
    pub index: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct Del {
    pub indexs: Vec<usize>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Favor {
    pub index: usize,
}

impl Save {
    pub fn new(arch_name: &str, arch_note: &str) -> Self {
        Self {
            arch_name: arch_name.to_string(),
            arch_note: arch_note.to_string(),
        }
    }
}

impl Modify {
    pub fn new(index: usize, arch_name: &str, arch_note: &str) -> Self {
        Self {
            index,
            info: Save::new(arch_name, arch_note),
        }
    }
}

impl Load {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl Del {
    pub fn new_slices(indexs: &[usize]) -> Self {
        Self { indexs: Vec::from(indexs) }
    }
    pub fn new_vec(indexs: Vec<usize>) -> Self {
        Self { indexs }
    }
}

impl Favor {
    pub fn new(index: usize) -> Self {
        Self {index}
    }
}