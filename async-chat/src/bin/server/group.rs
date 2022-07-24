use async_std::task;
use std::sync::Arc;
use tokio::sync::broadcast;
use async_chat::FromServer;
use tokio::sync::broadcast::error::RecvError;

use crate::connection::Outbound;

pub struct Group {
  name: Arc<String>,
  sender: broadcast::Sender<Arc<String>>
}

impl Group {
  pub fn new(name: Arc<String>) -> Group {
    let (sender, _) = broadcast::channel(1000);

    Group { name, sender }
  }

  pub fn join(&self, outbound: Arc<Outbound>) {
    let receiver = self.sender.subscribe();

    task::spawn(handle_subscriber(
      self.name.clone(), receiver, outbound));
  }

  pub fn post(&self, message: Arc<String>) {
    // This only returns an error when there are no subscribers. A
    // connection's outgoing side can exit, dropping its subscription,
    // slightly before its incoming side, which may end up trying to send a
    // message to an empty group.
    let _ = self.sender.send(message);
  }
}

async fn handle_subscriber(
  group_name: Arc<String>,
  mut receiver: broadcast::Receiver<Arc<String>>,
  outbound: Arc<Outbound>  
) {
  loop {
    let packet = match receiver.recv().await {
      Ok(message) => FromServer::Message{
        group_name: group_name.clone(),
        message: message.clone(),
      },

      Err(RecvError::Lagged(n)) => FromServer::Error(
        format!("Dropped {n} messages from {group_name}")
      ),

      Err(RecvError::Closed) => break,
    };

    if outbound.send(packet).await.is_err() {
      break;
    }
  }
}
