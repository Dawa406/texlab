mod client;
mod server;

pub use self::{
    client::{TestLatexLspClient, TestLspClient},
    server::TestLatexLspServer,
};

use crate::jsonrpc::MessageHandler;

pub struct TestBed {}
