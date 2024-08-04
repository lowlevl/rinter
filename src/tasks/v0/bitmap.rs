use std::{io::Cursor, sync::Arc};

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::Result,
};
use image::ImageReader;
use tokio::sync::mpsc;

use super::super::Message;
use crate::PRINTER_PIX_WIDTH;

pub async fn put(
    State(sender): State<Arc<mpsc::Sender<Message>>>,
    mut multipart: Multipart,
) -> Result<StatusCode> {
    while let Some(field) = multipart.next_field().await? {
        if field.name() != Some("file") {
            continue;
        }

        let mut data = vec![];

        let reader = ImageReader::new(Cursor::new(field.bytes().await?));
        reader
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .grayscale()
            .resize(
                PRINTER_PIX_WIDTH,
                u32::MAX,
                image::imageops::FilterType::Nearest,
            )
            .write_to(&mut Cursor::new(&mut data), image::ImageFormat::Jpeg)
            .unwrap();

        sender.send(Message::BitMap { data }).await.unwrap();
    }

    Ok(StatusCode::ACCEPTED)
}
