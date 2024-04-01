use thiserror::Error;

#[derive(Error, Debug)]
pub enum AcpError {
    #[error("Command line arguments error: {0}")]
    CommandLineArgs(#[from] CommandLineArgsError),
}

#[derive(Error, Debug)]
pub enum CommandLineArgsError {
    #[error("Password should be a large number")]
    PasswordShouldBeALargeNumber,
}
