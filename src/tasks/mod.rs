use std::sync::Arc;

use axum::{
    routing::{get, put},
    Router,
};
use escpos::{
    printer::Printer,
    utils::{BitImageOption, BitImageSize},
};
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
    let printer = printer.init()?.smoothing(true)?;

    loop {
        let Some(message) = rx.recv().await else {
            break;
        };

        // TODO: Maybe use `spawn_blocking` here.
        match message {
            Message::Text { data } => printer.writeln(&data)?,
            Message::BitMap { data } => printer.bit_image_from_bytes_option(
                &data,
                BitImageOption::new(None, None, BitImageSize::Normal)?,
            )?,
            Message::Cut { partial } => {
                printer.feeds(2)?;

                if partial {
                    printer.partial_cut()?
                } else {
                    printer.cut()?
                }
            }
        }
        .print()?;
    }

    Ok(())
}

pub async fn serve(listener: TcpListener, tx: mpsc::Sender<Message>) -> eyre::Result<()> {
    let v0 = Router::new()
        .route("/text", put(v0::text::put))
        .route("/bitmap", put(v0::bitmap::put))
        .route("/cut", put(v0::cut::put))
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
