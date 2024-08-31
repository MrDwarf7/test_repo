use std::sync::OnceLock;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub static NOW: OnceLock<std::time::Duration> = OnceLock::new();

pub const MAX_VALUE: usize = 10;

pub const HIGHEST_VALUE: usize = 100;
