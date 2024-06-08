use super::io_manager::Error;

#[derive(Debug,Clone)]
pub enum CommandID {
    IdErrCommand(Error),

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

    IdModarch(Option<Save>),
    IdDel(Option<Del>),
    IdQdel,

    IdUsage,
    IdFavor,
}

#[derive(Clone, Debug)]
pub struct Save {
    arch_name: String,
    arch_note: String,
}

#[derive(Clone, Debug)]
pub struct Load {
    index: usize,
}

#[derive(Clone, Debug)]
pub struct Del {
    index: usize,
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
