use thiserror::Error;
use tonic::Status;

pub trait StatusAsError {
    fn map_status_to_err(&self) -> AuthenticationError;
}

impl StatusAsError for Status {
    fn map_status_to_err(&self) -> AuthenticationError {
        AuthenticationError::UnableToAuthenticateWithServer {
            status: self.clone(),
        }
    }
}

#[derive(Error, Debug)]
pub enum AcpError {
    #[error("Authentication error: {0}")]
    Authentication(#[from] AuthenticationError),
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
}

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("Unable to authenticate with server: {status}")]
    UnableToAuthenticateWithServer { status: tonic::Status },
    #[error("Could not get password from user entry")]
    CouldNotGetPassword,
    #[error("Unable to get the authentication type from the server")]
    UnableToGetAuthTypeFromServer,
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Unable to find challange")]
    UnableToFindChallenge,
    #[error("Unable to find registration")]
    UnableToFindRegistration,
}
