use anyhow::Result;
use serde_json::json;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::info;

pub struct ApiServer {
    pub port: u16,
}

impl ApiServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn start(&self) -> Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await?;
        info!("Zephyx Internal REST API listener active at http://{}", addr);

        tokio::spawn(async move {
            loop {
                if let Ok((mut socket, _)) = listener.accept().await {
                    tokio::spawn(async move {
                        let mut buffer = [0u8; 1024];
                        let _ = socket.read(&mut buffer).await;

                        let body = json!({
                            "status": "online",
                            "version": "0.3.0",
                            "endpoints": [
                                "/api/v1/workspaces",
                                "/api/v1/tasks",
                                "/api/v1/findings",
                                "/api/v1/recommendations",
                                "/api/v1/workflow",
                                "/api/v1/plugins",
                                "/api/v1/reports",
                                "/api/v1/snapshots",
                                "/api/v1/objectives",
                                "/api/v1/hypotheses",
                                "/api/v1/strategies",
                                "/api/v1/reasoning",
                                "/api/v1/timeline",
                                "/api/v1/browser",
                                "/api/v1/statistics"
                            ]
                        }).to_string();

                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            body.len(),
                            body
                        );

                        let _ = socket.write_all(response.as_bytes()).await;
                    });
                }
            }
        });

        Ok(())
    }
}
