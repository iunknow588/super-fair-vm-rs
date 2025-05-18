use crate::core::types::{Address, U256, Hash};
use std::collections::HashMap;

pub struct StateDB {
    pub accounts: HashMap<Address, Account>,
    pub storage: HashMap<(Address, Hash), U256>,
    pub codes: HashMap<Address, Vec<u8>>,
}

#[derive(Default, Clone, Debug)]
pub struct Account {
    pub nonce: U256,
    pub balance: U256,
    // ... 其它字段
}

impl StateDB {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            storage: HashMap::new(),
            codes: HashMap::new(),
        }
    }

    pub fn get_account(&self, addr: &Address) -> Option<&Account> {
        self.accounts.get(addr)
    }

    pub fn set_account(&mut self, addr: Address, account: Account) {
        self.accounts.insert(addr, account);
    }

    pub fn get_storage(&self, addr: &Address, key: &Hash) -> Option<&U256> {
        self.storage.get(&(*addr, *key))
    }

    pub fn set_storage(&mut self, addr: Address, key: Hash, value: U256) {
        self.storage.insert((addr, key), value);
    }

    pub fn get_code(&self, addr: &Address) -> Option<&Vec<u8>> {
        self.codes.get(addr)
    }

    pub fn set_code(&mut self, addr: Address, code: Vec<u8>) {
        self.codes.insert(addr, code);
    }
}
