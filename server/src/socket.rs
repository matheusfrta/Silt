use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Payload {
    pub id: usize,
    pub v: f64,
}

pub struct Server {
    tx: broadcast::Sender<String>,
}

impl Server {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self { tx }
    }

    pub async fn run(&self, addr: &str) {
        let list = TcpListener::bind(addr).await.unwrap();
        loop {
            let (mut sock, _) = list.accept().await.unwrap();
            let mut rx = self.tx.subscribe();
            
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                loop {
                    tokio::select! {
                        Ok(msg) = rx.recv() => {
                            if sock.write_all(msg.as_bytes()).await.is_err() { break; }
                        }
                        Ok(n) = sock.read(&mut buf) => {
                            if n == 0 { break; }
                            // sync internal graph here if needed
                        }
                    }
                }
            });
        }
    }
    
    pub fn push(&self, id: usize, v: f64) {
        let p = Payload { id, v };
        let _ = self.tx.send(serde_json::to_string(&p).unwrap() + "\n");
    }
}