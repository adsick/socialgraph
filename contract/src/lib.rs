use std::collections::BTreeMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, AccountId, env};



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    connections: UnorderedMap<AccountId, BTreeMap<AccountId, (u8, u8)>>,
    // connections_rev: UnorderedMap<>,
    // familiarity: UnorderedMap<AccountId, HashSet<(AccountId, u8)>>, //knowledge map
    // dependancies: UnorderedMap<AccountId, HashSet<(AccountId, u8)>>,
    
    // know: UnorderedMap<AccountId, HashSet<(AccountId, u8)>>,
    // known_by: UnorderedSet<AccountId>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            connections: UnorderedMap::new(b"c")
        }
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn connect(&mut self, to: AccountId, distance: u8){
        let predecessor = env::predecessor_account_id();
        let mut connections = self.connections.get(&predecessor).unwrap_or_default();
        
        connections.insert(to, (1, distance));
        self.connections.insert(&predecessor, &connections);
    }

    pub fn get_connections_for(&self, account_id: AccountId)->BTreeMap<AccountId, (u8, u8)>{
        self.connections.get(&account_id).expect("not found")
    }
}