//use super::io_manager::Error;

#[derive(Debug, Clone)]
pub enum CommandID {
    IdErrCommand(String),

    IdClear,
    IdHelp,
    IdQuit,

    IdSave(Option<Save>),
    IdQsave(Option<Save>),
    IdRsave(Option<Save>),

    IdLoad(Option<Load>),
    IdQload(Option<Load>),
    IdLog,
    IdSlog,

    IdModarch(Option<Modify>),
    IdDel(Option<Del>),
    IdQdel,

    IdUsage,
    IdFavor,
}

#[derive(Clone, Debug)]
pub struct Save {
    pub arch_name: String,
    pub arch_note: String,
}

#[derive(Clone, Debug)]
pub struct Modify {
    pub index: usize,
    pub info: Save,
}

#[derive(Clone, Debug)]
pub struct Load {
    pub index: usize,
}

#[derive(Clone, Debug)]
pub struct Del {
    pub index: usize,
}

impl Save {
    pub fn new(arch_name: &str, arch_note: &str) -> Self {
        Self {
            arch_name: arch_name.to_string(),
            arch_note: arch_note.to_string(),
        }
    }
    pub fn get_empty() -> Self {
        Self {
            arch_name: "".to_string(),
            arch_note: "".to_string(),
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
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}
