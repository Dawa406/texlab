use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use futures::channel::mpsc;
use futures::prelude::*;
use jsonrpc::MessageHandler;
use std::error::Error;
use std::sync::Arc;
use stderrlog::{ColorChoice, Timestamp};
use texlab::server::LatexLspServer;
use texlab_distro::Distribution;
use texlab_protocol::{LatexLspClient, LspCodec};
use tokio_util::codec::{FramedRead, FramedWrite};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = app_from_crate!()
        .author("")
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Increase message verbosity"),
        )
        .arg(
            Arg::with_name("quiet")
                .long("quiet")
                .short("q")
                .help("No output printed to stderr"),
        )
        .get_matches();

    stderrlog::new()
        .module(module_path!())
        .module("jsonrpc")
        .module("texlab_citeproc")
        .module("texlab_completion")
        .module("texlab_distro")
        .module("texlab_hover")
        .module("texlab_protocol")
        .module("texlab_symbol")
        .module("texlab_syntax")
        .module("texlab_workspace")
        .verbosity(matches.occurrences_of("verbosity") as usize)
        .quiet(matches.is_present("quiet"))
        .timestamp(Timestamp::Off)
        .color(ColorChoice::Never)
        .init()
        .unwrap();

    let mut stdin = FramedRead::new(tokio::io::stdin(), LspCodec);
    let (stdout_tx, mut stdout_rx) = mpsc::channel(0);

    let client = Arc::new(LatexLspClient::new(stdout_tx.clone()));
    let server = Arc::new(LatexLspServer::new(
        Arc::clone(&client),
        Arc::new(Distribution::detect().await),
    ));
    let mut handler = MessageHandler {
        server,
        client,
        output: stdout_tx,
    };

    tokio::spawn(async move {
        let mut stdout = FramedWrite::new(tokio::io::stdout(), LspCodec);
        loop {
            let message = stdout_rx.next().await.unwrap();
            stdout.send(message).await.unwrap();
        }
    });

    while let Some(json) = stdin.next().await {
        handler.handle(&json.unwrap()).await;
    }

    Ok(())
}
