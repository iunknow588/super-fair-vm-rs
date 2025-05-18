use crate::account::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NFTStandard {
    ERC721,
    ERC1155,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTToken {
    pub token_id: u64,
    pub owner: Address,
    pub metadata: NFTMetadata,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTContract {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub standard: NFTStandard,
    pub tokens: HashMap<u64, NFTToken>,
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
        token_id: u64,
        owner: Address,
        metadata: NFTMetadata,
        uri: String,
    ) -> Result<(), String> {
        if self.tokens.contains_key(&token_id) {
            return Err("Token ID already exists".to_string());
        }

        self.tokens.insert(
            token_id,
            NFTToken {
                token_id,
                owner,
                metadata,
                uri,
            },
        );

        Ok(())
    }

    pub fn transfer(&mut self, token_id: u64, from: Address, to: Address) -> Result<(), String> {
        if let Some(token) = self.tokens.get_mut(&token_id) {
            if token.owner != from {
                return Err("Not the owner of the token".to_string());
            }
            token.owner = to;
            Ok(())
        } else {
            Err("Token does not exist".to_string())
        }
    }

    pub fn get_token(&self, token_id: u64) -> Option<&NFTToken> {
        self.tokens.get(&token_id)
    }

    pub fn get_tokens_by_owner(&self, owner: Address) -> Vec<&NFTToken> {
        self.tokens
            .values()
            .filter(|token| token.owner == owner)
            .collect()
    }
}
