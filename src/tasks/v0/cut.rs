use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Result,
};
use tokio::sync::mpsc;

use super::super::Message;

#[derive(serde::Deserialize)]
pub struct CutParams {
    #[serde(default)]
    partial: bool,
}

pub async fn put(
    State(sender): State<Arc<mpsc::Sender<Message>>>,
    Query(query): Query<CutParams>,
) -> Result<StatusCode> {
    sender
        .send(Message::Cut {
            partial: query.partial,
        })
        .await
        .unwrap();

    Ok(StatusCode::ACCEPTED)
}
