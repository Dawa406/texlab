use crate::{
    jsonrpc::server::{Middleware, Result},
    protocol::*,
};
use futures::lock::Mutex;
use futures_boxed::boxed;
use jsonrpc_derive::{jsonrpc_method, jsonrpc_server};

#[derive(Debug)]
pub struct TestLatexLspServer {
    pub options: Mutex<Options>,
    pub show_message_buf: Mutex<Vec<ShowMessageParams>>,
    pub register_cap_buf: Mutex<Vec<RegistrationParams>>,
    pub progress_buf: Mutex<Vec<ProgressParams>>,
    pub log_message_buf: Mutex<Vec<LogMessageParams>>,
}

#[jsonrpc_server]
impl TestLatexLspServer {
    pub fn new(options: Options) -> Self {
        Self {
            options: Mutex::new(options),
            show_message_buf: Mutex::default(),
            register_cap_buf: Mutex::default(),
            progress_buf: Mutex::default(),
            log_message_buf: Mutex::default(),
        }
    }

    #[jsonrpc_method("workspace/configuration", kind = "request")]
    pub async fn configuration(&self, _params: ConfigurationParams) -> Result<serde_json::Value> {
        let options = self.options.lock().await;
        Ok(serde_json::to_value::<&Options>(&options).unwrap())
    }

    #[jsonrpc_method("window/showMessage", kind = "notification")]
    pub async fn show_message(&self, params: ShowMessageParams) {
        let mut buf = self.show_message_buf.lock().await;
        buf.push(params);
    }

    #[jsonrpc_method("client/registerCapability", kind = "request")]
    pub async fn register_capability(&self, params: RegistrationParams) -> Result<()> {
        let mut buf = self.register_cap_buf.lock().await;
        buf.push(params);
        Ok(())
    }
}

impl Middleware for TestLatexLspServer {
    #[boxed]
    async fn before_message(&self) {}

    #[boxed]
    fn after_message(&self) {}
}
