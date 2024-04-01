use thiserror::Error;

#[derive(Error, Debug)]
pub enum AcpError {
    #[error("An unknown error has occured")]
    Unknown,
}
