use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use tokio::sync::mpsc;

use super::super::Message;

#[derive(serde::Deserialize)]
pub struct CutParams {
    partial: Option<bool>,
}

pub async fn cut(
    State(sender): State<Arc<mpsc::Sender<Message>>>,
    Query(query): Query<CutParams>,
) -> impl IntoResponse {
    sender
        .send(Message::Cut {
            partial: query.partial.unwrap_or_default(),
        })
        .await;

    "Cut\n"
}
