use crate::cache::split::{Split, Structure};

pub enum RunType {
    Bastionless,
    Modern,
}

pub struct RunInfo {
    pub split: Split,
    pub structure: Option<Structure>,
    pub run_type: RunType,
}
