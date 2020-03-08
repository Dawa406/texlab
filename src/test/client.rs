use crate::{jsonrpc::client::Result, protocol::*};
use futures_boxed::boxed;
use jsonrpc_derive::{jsonrpc_client, jsonrpc_method};

#[jsonrpc_client(TestLatexLspClient)]
pub trait TestLspClient {
    #[jsonrpc_method("initialize", kind = "request")]
    #[boxed]
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult>;

    #[jsonrpc_method("initialized", kind = "notification")]
    #[boxed]
    async fn initialized(&self, params: InitializedParams);
}
