use std::sync::Arc;
use async_std::prelude::*;
use crate::{self, ChatResult, Client};
use async_std::{io, net};
use crate::utilities::ChatResult;

async fn send(mut send: net::TcpStream) {
    println!("join Chat\nor\nsend Chat Message...");
    let mut options = io::BufReader::new(io::stdin()).lines();

    while let Some(option_result) = options.next().await {
        let opt = option_result?;
        let req = match parse_input(&opt) {
            Some(req) => req,
            None => continue,
        };

        crate::utilities::send_json(&mut send, &req).await?;
        send.flush().await?;
    }
}

fn get_value(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();
    if input.is_empty() {
        return None;
    }

    match input.find(char::is_whitespace) {
        Some(whitespace) => {Some((&input[0..whitespace], &input[whitespace..]))}
        None =>{Some((input, ""))}
    }

}

fn parse_input(line: &str) -> Option<Client> {
    let (input, remain) = get_value(line)?;
    if input == "join" {
        let (chat, remain) = get_value(remain)?;
        if remain.trim_start().is_empty() {
            return None;
        }
        return Some(Client::Join {
            chat_name: Arc::new(chat.to_string()),
        });
    } else if input == "post" {
        let (chat, remain) = get_value(remain)?;
        let message = remain.trim_start().to_string();
        return Some(Client::Post {
            chat_name: Arc::new(chat.to_string()),
            message: Arc::new(message),
        });
    } else {
        println!("Unrecognized {:?}", line);
        return None;
    }

}