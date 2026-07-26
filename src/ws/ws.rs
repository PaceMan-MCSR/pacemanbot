use std::{error::Error, sync::Arc, time::Duration};

use serenity::{
    client::Context,
    futures::{lock::Mutex, stream::FusedStream},
};
use tokio::{net::TcpStream, time::sleep};
use tokio_stream::StreamExt;
use tokio_tungstenite::{
    tungstenite::{handshake::client::generate_key, http::request},
    MaybeTlsStream, WebSocketStream,
};

use crate::{
    cache::Cache,
    dispatcher::Dispatcher,
    log::Log,
    ws::{
        WSResponse, WS_CONNECTION_HEADER, WS_SEC_VERSION_HEADER, WS_TIMEOUT_FOR_RETRY,
        WS_UPGRADE_HEADER,
    },
};

pub struct WS {
    pub url: String,
    pub host: String,
    pub auth_key: String,
}

impl WS {
    pub fn new(url: String, host: String, auth_key: String) -> Self {
        Self {
            url,
            host,
            auth_key,
        }
    }

    pub async fn connect(
        &self,
    ) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn Error>> {
        let request = request::Request::builder()
            .uri(self.url.to_owned())
            .header("auth", self.auth_key.to_owned())
            .header("sec-websocket-key", generate_key())
            .header("host", self.host.to_owned())
            .header("upgrade", WS_UPGRADE_HEADER)
            .header("connection", WS_CONNECTION_HEADER)
            .header("sec-websocket-version", WS_SEC_VERSION_HEADER)
            .body(())
            .unwrap();
        let (response_stream, _) = match tokio_tungstenite::connect_async(request).await {
            Ok(stream_tuple) => stream_tuple,
            Err(err) => return Err(err.into()),
        };
        Ok(response_stream)
    }

    pub async fn get_next(
        &self,
        stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    ) -> Result<WSResponse, Box<dyn Error>> {
        let msg = match match stream.next().await {
            Some(msg_result) => msg_result,
            None => {
                return Err(format!("websocket closed unexpectedly.").into());
            }
        } {
            Ok(msg) => msg,
            Err(err) => {
                return Err(err.into());
            }
        };
        let text_response = match msg.to_text() {
            Ok(text) => text,
            Err(err) => {
                return Err(err.into());
            }
        };
        let response = match serde_json::from_str::<WSResponse>(text_response) {
            Ok(response) => response,
            Err(err) => {
                return Err(err.into());
            }
        };
        Ok(response)
    }

    pub async fn start_event_loop(
        &self,
        ctx: Arc<Context>,
        log: Arc<Log>,
        cache: Arc<Mutex<Cache>>,
    ) {
        loop {
            let mut stream = match self.connect().await {
                Ok(stream) => stream,
                Err(err) => {
                    log.error(format!("Websocket connect error: {}", err).as_str());
                    continue;
                }
            };
            loop {
                let response = match self.get_next(&mut stream).await {
                    Ok(response) => response,
                    Err(err) => {
                        log.error(format!("Websocket get next error: {}", err).as_str());
                        if stream.is_terminated() {
                            break;
                        }
                        continue;
                    }
                };
                let dispatcher = Dispatcher::new(ctx.clone(), log.clone(), cache.clone(), response);
                match dispatcher.dispatch().await {
                    Ok(_) => (),
                    Err(err) => {
                        log.error(format!("Failed to dispatch pace due to: {}", err).as_str());
                    }
                };
            }
            sleep(Duration::from_secs(WS_TIMEOUT_FOR_RETRY)).await;
        }
    }
}
