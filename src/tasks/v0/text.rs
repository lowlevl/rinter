use std::sync::Arc;

use axum::{
    extract::State,
    response::IntoResponse,
};
use tokio::sync::mpsc;

use super::super::Message;

pub async fn text(State(sender): State<Arc<mpsc::Sender<Message>>>) -> impl IntoResponse {
    sender
        .send(Message::Text {
            data: "test".into(),
        })
        .await;

    "Printed\n"
}
