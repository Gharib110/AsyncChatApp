use std::sync::Arc;
use async_std::{net, task};
use async_std::prelude::*;
use AsyncChatApp::chats::Chats;
use AsyncChatApp::chats_map::ChatTracker;
use AsyncChatApp::connection::handle;
use AsyncChatApp::utilities::ChatResult;

fn main() {
    let addr = std::env::args().nth(1).expect("Server Address");
    let chat_table = Arc::new(ChatTracker::new());

    async_std::task::block_on(async {
        let listener = net::TcpListener::bind(addr).await?;
        let mut new_connection = listener.incoming();
        while let Some(socket_result) = new_connection.next().await {
            let socket = socket_result?;
            let chats = chat_table.clone();
            task::spawn(async {
                log_error(handle(socket, chats).await)
            })
        }
    })
}

fn log_error(result: ChatResult<()>) {
    if let Err(err) = result {
        println!("Error : {}", err);
    }
}