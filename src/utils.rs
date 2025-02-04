use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("System detection failure")]
    DetectionError,
    #[error("Driver management error")]
    DriverError(#[from] crate::drivers::DriverError),
}

pub type Result<T> = std::result::Result<T, AppError>;

pub fn init_logging() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
}