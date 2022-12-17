use crate::connection::Leaving;
use async_std::task;
use chat::Server;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

pub struct Chats {
    name: Arc<String>,
    publisher: Sender<Arc<String>>,
}

impl Chats {
    pub fn new(name: Arc<String>) -> Chats {
        let (publisher, _): (Sender<Arc<String>>, Receiver<Arc<String>>) = broadcast::channel(1000);
        Chats { name, publisher }
    }
    pub fn join(&self, leaving: Arc<Leaving>) -> () {
        let receiver: Receiver<Arc<String>> = self.publisher.subscribe();
        task::spawn(sub(self.name.clone(), receiver, leaving));
    }
    pub fn post(&self, message: Arc<String>) -> () {
        let _ = self.publisher.send(message);
    }
}

async fn sub(chat_name: Arc<String>, mut receiver: Receiver<Arc<String>>, leaving: Arc<Leaving>) {
    loop {
        let packet: Server = match receiver.recv().await {
            Ok(message) => Server::Message {
                chat_name: chat_name.clone(),
                message: message.clone(),
            },
            Err(RecvError::Lagged(n)) => {
                Server::Error(format!("Dropped {} messages from {}.", n, chat_name))
            }
            Err(RecvError::Closed) => break,
        };
        if leaving.send(packet).await.is_err() {
            break;
        }
    }
}