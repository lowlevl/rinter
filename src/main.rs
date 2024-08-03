use std::net::SocketAddr;

use clap::Parser;
use clap_num::maybe_hex;
use escpos::{driver::NativeUsbDriver, printer::Printer};
use tokio::{net::TcpListener, sync::mpsc};

mod tasks;

/// A little server-side application to control an ESC/POS printer.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Server-side application's listen port.
    addr: SocketAddr,

    /// Printer vendor ID.
    #[clap(value_parser = maybe_hex::<u16>)]
    vid: u16,

    /// Printer product ID.
    #[clap(value_parser = maybe_hex::<u16>)]
    pid: u16,
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
        tasks::act(printer, rx),
        tasks::serve(listener, tx),
    }?;

    Ok(())
}
