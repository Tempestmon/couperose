use crate::grpc_client::AppState;
use actix_web::{get, web::Data, web::Payload, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use tracing::info;

#[get("/ws")]
pub async fn ws_handler(
    req: HttpRequest,
    stream: Payload,
    state: Data<AppState>,
) -> actix_web::Result<HttpResponse> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;
    let mut rx = state.broadcast_tx.subscribe();

    info!("WebSocket client connected");

    actix_web::rt::spawn(async move {
        loop {
            tokio::select! {
                msg = rx.recv() => {
                    match msg {
                        Ok(text) => {
                            if session.text(text).await.is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
                msg = msg_stream.next() => {
                    match msg {
                        Some(Ok(actix_ws::Message::Close(_))) | None => break,
                        _ => {}
                    }
                }
            }
        }
        let _ = session.close(None).await;
        info!("WebSocket client disconnected");
    });

    Ok(response)
}
