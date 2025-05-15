pub mod chain_handlers;
pub mod static_handlers;
pub mod wallet_handlers;

use crate::vm::VM;
use jsonrpc_core::Error;
use serde_json;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ApiServer {
    vm: Arc<RwLock<VM>>,
}

impl ApiServer {
    pub fn new(vm: Arc<RwLock<VM>>) -> Self {
        Self { vm }
    }

    pub fn chain_handlers(&self) -> chain_handlers::ChainHandlers {
        chain_handlers::ChainHandlers::new(self.vm.clone())
    }

    pub fn static_handlers(&self) -> static_handlers::StaticHandlers {
        static_handlers::StaticHandlers::new(self.vm.clone())
    }

    pub fn wallet_handlers(&self) -> wallet_handlers::WalletHandlers {
        wallet_handlers::WalletHandlers::new(self.vm.clone())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("VM错误: {0}")]
    VmError(String),
    #[error("无效参数: {0}")]
    InvalidParams(String),
    #[error("内部错误: {0}")]
    Internal(String),
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        match e {
            ApiError::VmError(msg) => {
                let mut err = Error::internal_error();
                err.data = Some(serde_json::Value::String(msg));
                err
            }
            ApiError::InvalidParams(msg) => Error::invalid_params(msg),
            ApiError::Internal(msg) => {
                let mut err = Error::internal_error();
                err.data = Some(serde_json::Value::String(msg));
                err
            }
        }
    }
}
