use super::{DaemonConnection, request};
use anyhow::Context;
use async_trait::async_trait;
use tokio::{
    io::BufReader,
    net::{TcpStream, ToSocketAddrs},
};
use tracing::info;

pub struct TcpConnection {
    inner: BufReader<TcpStream>,
}

impl TcpConnection {
    pub async fn connect(addr: impl ToSocketAddrs) -> anyhow::Result<Box<Self>> {
        info!("connecting to remote TCP service");
        let inner = TcpStream::connect(addr).await?;
        Ok(Box::new(Self {
            inner: BufReader::new(inner),
        }))
    }
}
#[async_trait]
impl DaemonConnection for TcpConnection {
    async fn request<'a>(&'a mut self, payload: &'a str) -> anyhow::Result<String> {
        request(&mut self.inner, payload).await 
    }

    async fn new_connection(&self) -> anyhow::Result<Box<dyn DaemonConnection>> {
            let peer_addr = self
                .inner
                .get_ref()
                .peer_addr()
                .context("Could not read peer address")?;

            Ok(Self::connect(peer_addr).await? as Box<dyn DaemonConnection>)
    }
}
