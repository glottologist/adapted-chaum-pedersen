use thiserror::Error;
use tonic::Status;

// Trait to convert a tonic::Status into an AuthenticationError
pub trait StatusAsError {
    fn map_status_to_err(&self) -> AuthenticationError;
}

impl StatusAsError for Status {
    fn map_status_to_err(&self) -> AuthenticationError {
        // Specifically maps to UnableToAuthenticateWithServer variant, including the status
        AuthenticationError::UnableToAuthenticateWithServer {
            status: self.clone(),
        }
    }
}

// Define custom error types using the thiserror crate for clearer error handling and propagation
#[derive(Error, Debug)]
pub enum AcpError {
    // Authentication error variant, encapsulating an AuthenticationError
    #[error("Authentication error: {0}")]
    Authentication(#[from] AuthenticationError),
    // Storage error variant, encapsulating a StorageError
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
}

// Define authentication error variants
#[derive(Error, Debug)]
pub enum AuthenticationError {
    // Error variant for authentication issues, includes tonic::Status for more context
    #[error("Unable to authenticate with server: {status}")]
    UnableToAuthenticateWithServer { status: tonic::Status },
    // Error variant for issues retrieving passwords from user entries
    #[error("Could not get password from user entry")]
    CouldNotGetPassword,
    // Error variant for failures in getting the authentication type from the server
    #[error("Unable to get the authentication type from the server")]
    UnableToGetAuthTypeFromServer,
}

// Define storage error variants
#[derive(Error, Debug)]
pub enum StorageError {
    // Error variant for failing to find a challenge
    #[error("Unable to find challange")]
    UnableToFindChallenge,
    // Error variant for failing to find a registration
    #[error("Unable to find registration")]
    UnableToFindRegistration,
}
