use crate::vm::VM;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use std::sync::Arc;
use tokio::sync::RwLock;

#[rpc]
pub trait StaticApi {
    #[rpc(name = "fairvm_ping")]
    fn ping(&self) -> Result<String>;

    #[rpc(name = "fairvm_version")]
    fn version(&self) -> Result<String>;

    #[rpc(name = "fairvm_networkId")]
    fn network_id(&self) -> Result<String>;

    #[rpc(name = "fairvm_chainId")]
    fn chain_id(&self) -> Result<String>;
}

pub struct StaticHandlers {
    #[allow(dead_code)]
    vm: Arc<RwLock<VM>>,
}

impl StaticHandlers {
    pub fn new(vm: Arc<RwLock<VM>>) -> Self {
        Self { vm }
    }
}

impl StaticApi for StaticHandlers {
    fn ping(&self) -> Result<String> {
        Ok("pong".to_string())
    }

    fn version(&self) -> Result<String> {
        Ok(env!("CARGO_PKG_VERSION").to_string())
    }

    fn network_id(&self) -> Result<String> {
        Ok("fairvm".to_string())
    }

    fn chain_id(&self) -> Result<String> {
        Ok("0x1".to_string()) // 使用以太坊主网chainId作为示例
    }
}
