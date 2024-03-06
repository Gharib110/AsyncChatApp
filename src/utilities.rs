use std::error::Error;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;

pub type ChatResult<T> = Result<T, ChatError>;