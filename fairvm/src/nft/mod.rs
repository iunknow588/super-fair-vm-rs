use crate::account::Address;
use ethers::types::U256;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// NFT标准类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NFTStandard {
    ERC721,
    ERC1155,
}

/// NFT元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}

/// NFT属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

/// NFT代币
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTToken {
    pub id: U256,
    pub owner: Address,
    pub metadata: NFTMetadata,
    pub standard: NFTStandard,
    pub supply: U256,
}

/// NFT合约
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTContract {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub standard: NFTStandard,
    pub tokens: HashMap<U256, NFTToken>,
}

impl NFTContract {
    pub fn new(address: Address, name: String, symbol: String, standard: NFTStandard) -> Self {
        Self {
            address,
            name,
            symbol,
            standard,
            tokens: HashMap::new(),
        }
    }

    pub fn mint(
        &mut self,
        token_id: U256,
        owner: Address,
        metadata: NFTMetadata,
        supply: U256,
    ) -> Result<(), String> {
        if self.tokens.contains_key(&token_id) {
            return Err("Token already exists".to_string());
        }

        let token = NFTToken {
            id: token_id,
            owner,
            metadata,
            standard: self.standard,
            supply,
        };

        self.tokens.insert(token_id, token);
        Ok(())
    }

    pub fn transfer(
        &mut self,
        token_id: U256,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<(), String> {
        let token = self.tokens.get_mut(&token_id).ok_or("Token not found")?;

        if token.owner != from {
            return Err("Not token owner".to_string());
        }

        if token.supply < amount {
            return Err("Insufficient balance".to_string());
        }

        token.owner = to;
        token.supply -= amount;
        Ok(())
    }

    pub fn get_token(&self, token_id: U256) -> Option<&NFTToken> {
        self.tokens.get(&token_id)
    }

    pub fn get_token_mut(&mut self, token_id: U256) -> Option<&mut NFTToken> {
        self.tokens.get_mut(&token_id)
    }

    pub fn get_tokens_by_owner(&self, owner: Address) -> Vec<&NFTToken> {
        self.tokens
            .values()
            .filter(|token| token.owner == owner)
            .collect()
    }
}
