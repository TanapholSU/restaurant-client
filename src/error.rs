use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError{
    #[error("Json request serialization error")]
    SerializationError,
    
    #[error("Request communication error")]
    RequestError
}