use async_std::net;
use async_std::net::Incoming;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::StreamExt;
use async_std::task;
use chat::utils::ChatError;
use chat::utils::ChatResult;
use std::future::Future;
use std::sync::Arc;

mod chats;
mod chats_map;
mod connection;

use crate::chats_map::ChatTracker;
use connection::handle;

fn log_error(result: ChatResult<()>) {
    if let Err(error) = result {
        println!("Error {}", error);
    }
}

fn main() -> ChatResult<()> {
    let addr: String = std::env::args().nth(1).expect("server ADDRESS:PORT");
    let chat_table: Arc<ChatTracker> = Arc::new(ChatTracker::new());
    let future = async {
        let listener: TcpListener = TcpListener::bind(addr).await?;
        let mut new_connections: Incoming = listener.incoming();
        while let Some(socket_result) = new_connections.next().await {
            let socket: TcpStream = socket_result?;
            let chats: Arc<ChatTracker> = chat_table.clone();
            task::spawn(async {
                log_error(handle(socket, chats).await);
            });
        }
        Ok(())
    };
    async_std::task::block_on(future)
}
