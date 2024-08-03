use std::sync::Arc;

use axum::{
    routing::{get, put},
    Router,
};
use escpos::printer::Printer;
use tokio::{net::TcpListener, sync::mpsc};

mod v0;

pub enum Message {
    Text { data: String },
    BitMap { data: Vec<u8> },
    Cut { partial: bool },
}

pub async fn act<D: escpos::driver::Driver>(
    mut printer: Printer<D>,
    mut rx: mpsc::Receiver<Message>,
) -> eyre::Result<()> {
    let mut printer = printer.init()?.smoothing(true)?;

    loop {
        let Some(message) = rx.recv().await else {
            break;
        };

        // TODO: Maybe use `spawn_blocking` here.
        printer = match message {
            Message::Text { data } => printer.writeln(&data)?,
            Message::BitMap { data } => printer.bit_image_from_bytes(&data)?,
            Message::Cut { partial: true } => printer.partial_cut()?,
            Message::Cut { partial: false } => printer.cut()?,
        };
        printer = printer.print()?;
    }

    Ok(())
}

pub async fn serve(listener: TcpListener, tx: mpsc::Sender<Message>) -> eyre::Result<()> {
    let v0 = Router::new()
        .route("/text", put(v0::text::text))
        .route("/cut", put(v0::cut::cut))
        .with_state(Arc::new(tx));

    axum::serve(
        listener,
        Router::new()
            .route("/", get(|| async { "Hello PertuiTek !" }))
            .nest("/v0", v0),
    )
    .await
    .map_err(Into::into)
}
