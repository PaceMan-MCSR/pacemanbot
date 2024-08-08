use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub mod consts;
pub mod response;
pub mod ws;

pub struct WSManager {
    pub stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}
