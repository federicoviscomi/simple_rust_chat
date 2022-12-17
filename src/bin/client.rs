use async_std::io::{BufReader, Lines, Stdin};
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use async_std::{io, net};
use chat::utils::{self, ChatError, ChatResult};
use chat::{Client, Server};
use std::ptr::addr_of_mut;
use std::sync::Arc;

async fn send(mut send: TcpStream) -> ChatResult<()> {
    println!("Options \njoin CHAT\npost CHAT MESSAGE\nType CTRL-C to close the connection.");
    let mut options: Lines<BufReader<Stdin>> = BufReader::new(io::stdin()).lines();
    while let Some(option_result) = options.next().await {
        let opt: String = option_result?;
        let req: Client = match parse_input(&opt) {
            Some(req) => req,
            None => continue,
        };
        utils::send_json(&mut send, &req).await?;
        send.flush().await?;
    }
    Ok(())
}

async fn messages(server: TcpStream) -> ChatResult<()> {
    let buf: BufReader<TcpStream> = BufReader::new(server);
    let mut stream = utils::receive(buf);
    while let Some(msg) = stream.next().await {
        match msg? {
            Server::Message { chat_name, message } => {
                println!("\n{:?}\t{:?} ", chat_name, message);
            }
            Server::Error(message) => {
                println!("Error {:?}", message);
            }
        }
    }
    Ok(())
}

fn get_value(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();
    if input.is_empty() {
        return None;
    }
    match input.find(char::is_whitespace) {
        Some(whitespace) => Some((&input[0..whitespace], &input[whitespace..])),
        None => Some((input, "")),
    }
}

fn parse_input(line: &str) -> Option<Client> {
    let (input, remainder): (&str, &str) = get_value(line)?;
    if input == "join" {
        let (chat, remainder): (&str, &str) = get_value(remainder)?;
        if !remainder.trim_start().is_empty() {
            return None;
        }
        return Some(Client::Join {
            chat_name: Arc::new(chat.to_string()),
        });
    } else if input == "post" {
        let (chat, remainder): (&str, &str) = get_value(remainder)?;
        let message: String = remainder.trim_start().to_string();
        return Some(Client::Post {
            chat_name: Arc::new(chat.to_string()),
            message: Arc::new(message),
        });
    } else {
        println!("Unrecognized input {:?}", line);
        None
    }
}

fn main() -> ChatResult<()> {
    let addr: String = std::env::args().nth(1).expect("ADDRESS:PORT");
    task::block_on(async {
        let socket: TcpStream = TcpStream::connect(addr).await?;
        socket.set_nodelay(true)?;
        let send = send(socket.clone());
        let replies = messages(socket);
        replies.race(send).await?;
        Ok(())
    })
}
