use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use clap::Parser;
use escpos::{driver::NativeUsbDriver, printer::Printer};
use tokio::{
    net::TcpListener,
    sync::{mpsc, Mutex},
};

/// A little server-side application to control an ESC/POS printer.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Server-side application's listen port.
    addr: SocketAddr,

    /// Printer vendor ID.
    vid: u16,

    /// Printer product ID.
    pid: u16,
}

enum Message {
    Text { data: String },
    Bitmap { data: Vec<u8> },
    Cut { partial: bool },
}

async fn act<D: escpos::driver::Driver>(
    printer: Printer<D>,
    rx: mpsc::Receiver<Message>,
) -> eyre::Result<()> {
    loop {}
}

async fn serve(listener: TcpListener, tx: mpsc::Sender<Message>) -> eyre::Result<()> {
    axum::serve(
        listener,
        Router::new()
            .with_state(Arc::new(tx))
            .route("/", get(|| async { "Hello, World!" })),
    )
    .await
    .map_err(Into::into)
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let listener = TcpListener::bind(args.addr).await?;
    let printer = Printer::new(
        NativeUsbDriver::open(args.vid, args.pid)?,
        Default::default(),
        None,
    );
    let (tx, rx) = mpsc::channel(1);

    tokio::try_join! {
        act(printer, rx),
        serve(listener, tx),
    }?;

    Ok(())
}
