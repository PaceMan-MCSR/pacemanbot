use std::env;
use std::time::Duration;

use crate::ws::consts::{WS_FALLBACK_HOST, WS_FALLBACK_URL, WS_TIMEOUT_FOR_RETRY};
use crate::{eprintln, Result};
use serenity::futures::StreamExt;
use tokio::net::TcpStream;
use tokio::time::sleep;
use tokio_tungstenite::{
    tungstenite::{handshake::client::generate_key, http::request},
    MaybeTlsStream, WebSocketStream,
};

use super::response::Response;
use super::{
    consts::{WS_CONNECTION_HEADER, WS_SEC_VERSION_HEADER, WS_UPGRADE_HEADER},
    WSManager,
};

impl WSManager {
    async fn print_err_and_sleep(response: String) {
        eprintln!("WSManager get next error: {}", response);
        println!("Trying again in {} seconds...", WS_TIMEOUT_FOR_RETRY);
        sleep(Duration::from_secs(WS_TIMEOUT_FOR_RETRY)).await;
    }

    pub async fn get_next(&mut self) -> Option<Response> {
        let msg = match match self.stream.next().await {
            Some(msg_result) => msg_result,
            None => {
                let response =
                    "Get Message Result Error: Invalid response from response stream.".to_string();
                Self::print_err_and_sleep(response).await;
                return None;
            }
        } {
            Ok(msg) => msg,
            Err(err) => {
                let response = format!("Get Message Error: {}", err);
                Self::print_err_and_sleep(response).await;
                return None;
            }
        };
        let text_response = match msg.to_text() {
            Ok(text) => text,
            Err(err) => {
                let response = format!("Get Text Response Error: {}", err);
                Self::print_err_and_sleep(response).await;
                return None;
            }
        };
        let response: Response = match serde_json::from_str(text_response) {
            Ok(response) => response,
            Err(err) => {
                let response = format!("JSONify error: {}", err);
                Self::print_err_and_sleep(response).await;
                return None;
            }
        };
        Some(response)
    }

    async fn get_response_stream() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let url = match env::var("WS_URL") {
            Ok(url) => url,
            Err(err) => {
                eprintln!("{}", err);
                WS_FALLBACK_URL.to_string()
            }
        };
        let host = match env::var("WS_HOST") {
            Ok(host) => host,
            Err(err) => {
                eprintln!("{}", err);
                WS_FALLBACK_HOST.to_string()
            }
        };
        let auth_key: String = match env::var("API_AUTH_KEY") {
            Ok(key) => key,
            Err(err) => return Err(format!("API_AUTH_KEY not found in env: {}", err).into()),
        };
        let request = request::Request::builder()
            .uri(url)
            .header("auth", auth_key.to_owned())
            .header("sec-websocket-key", generate_key())
            .header("host", host)
            .header("upgrade", WS_UPGRADE_HEADER)
            .header("connection", WS_CONNECTION_HEADER)
            .header("sec-websocket-version", WS_SEC_VERSION_HEADER)
            .body(())
            .unwrap();
        let (response_stream, _) = match tokio_tungstenite::connect_async(request).await {
            Ok(stream_tuple) => stream_tuple,
            Err(err) => return Err(format!("WS Connection error: {}", err).into()),
        };
        Ok(response_stream)
    }

    pub async fn new() -> std::result::Result<Self, String> {
        let stream = match Self::get_response_stream().await {
            Ok(stream) => stream,
            Err(err) => {
                let response = format!("{}", err);
                return Err(response.into());
            }
        };
        Ok(Self { stream })
    }
}
