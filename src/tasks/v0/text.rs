use std::sync::Arc;

use axum::{
    extract::{Multipart, Query, State},
    http::StatusCode,
    response::Result,
};
use tokio::sync::mpsc;

use super::super::Message;
use crate::PRINTER_CHAR_WIDTH;

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(serde::Deserialize)]
pub struct TextQuery {
    #[serde(default)]
    align: TextAlign,
}

pub async fn put(
    State(sender): State<Arc<mpsc::Sender<Message>>>,
    Query(query): Query<TextQuery>,
    mut multipart: Multipart,
) -> Result<StatusCode> {
    while let Some(field) = multipart.next_field().await? {
        if field.name() != Some("message") {
            continue;
        }

        for chunk in field
            .bytes()
            .await?
            .split(|c| *c == b'\n')
            .flat_map(|line| line.chunks(PRINTER_CHAR_WIDTH))
            .into_iter()
            .map(String::from_utf8_lossy)
        {
            let data = match query.align {
                TextAlign::Left => format!("{chunk:<0$}", PRINTER_CHAR_WIDTH),
                TextAlign::Center => format!("{chunk:^0$}", PRINTER_CHAR_WIDTH),
                TextAlign::Right => format!("{chunk:>0$}", PRINTER_CHAR_WIDTH),
            };

            sender.send(Message::Text { data }).await.unwrap();
        }
    }

    Ok(StatusCode::ACCEPTED)
}
