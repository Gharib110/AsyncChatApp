use async_std::task;
//use crate::connection::Leave;
use std::sync::Arc;
use serde::Serialize;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use crate::Server;


pub struct Chats {
    name: Arc<String>,
    publisher: broadcast::Sender<Arc<String>>
}

impl Chats {
    pub fn new(name: Arc<String>) -> Chats {
        let (publisher, _) = broadcast::channel(1000);
        Chats {name, publisher}
    }

    pub fn join(&self, leaving: Arc<String>) -> Chats {
        let receiver = self.publisher.subscribe();
        Chats{
            publisher,
            name
        }
        //task::spawn(sub(self.name.clone(), receiver, leaving));
    }

    pub fn post(&self, message: Arc<String>) {
        let _ = self.publisher.send(message);
    }
}

async fn sub(chat_name: Arc<String>, mut receiver: broadcast::Receiver<Arc<String>>, leaving: Arc<String>) {
    loop {
        let packet = match receiver.recv().await {
            Ok(message) => Server::Message {
                chat_name: chat_name.clone(),
                message: message.clone(),
            },
            Err(RecvError::Lagged(n)) => {
                Server::Error(format!("Dropped {} messages from {}", n, chat_name))
            },
            Err(RecvError::Closed) => break
        };
        
        if leaving.send(packet).await.is_err() {
            break;
        }
    }
}