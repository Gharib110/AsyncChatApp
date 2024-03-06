use async_std::prelude::*;
use serde::Serialize;
use std::error::Error;
use std::marker::Unpin;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;

pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_json<O, P>(leave: &mut O, data: &P) -> ChatResult<()>
where
    O: async_std::io::Write + Unpin,
    P: Serialize,
{
    let mut json:String = serde_json::to_string(&data)?;
    json.push('\n');
    leave.write_all(json.as_bytes()).await?;
    Ok(())
}
