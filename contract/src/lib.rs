use std::collections::BTreeMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, AccountId, env};



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SocialGraph {
    connections: UnorderedMap<AccountId, BTreeMap<String, u8>>,
}

// #[derive(BorshDeserialize, BorshSerialize)]
// pub struct ContractOld {
//     connections: UnorderedMap<AccountId, BTreeMap<AccountId, (u8, u8)>>,
// }

impl Default for SocialGraph {
    fn default() -> Self {
        Self {
            connections: UnorderedMap::new(b"c")
        }
    }
}

#[near_bindgen]
impl SocialGraph {
    #[payable]
    pub fn connect(&mut self, to: AccountId, kind: Option<String>, distance: u8){
        let predecessor = env::predecessor_account_id();
        let mut connections = self.connections.get(&predecessor).unwrap_or_default();
        let mut connection = to.to_string();
        if let Some(kind) = kind{
            connection += ":";
            connection += &kind;
        }
        connections.insert(connection, distance);
        self.connections.insert(&predecessor, &connections);
    }

    #[payable]
    pub fn disconnect(&mut self, from: AccountId, kind: Option<String>)->u8{
        let predecessor = env::predecessor_account_id();
        if let Some(mut connections) = self.connections.get(&predecessor){
            let mut connection = from.to_string();
            if let Some(kind) = kind{
                connection += ":";
                connection += &kind;
            }
            return connections.remove(&connection).expect("there was no connection")
        }
        env::panic_str("there is no connections for this user")
    }

    pub fn get_connections_for(&self, account_id: AccountId)->BTreeMap<String, u8>
    {
        // env::log_str(&format!("{:?}", self.connections.get(&account_id).unwrap()));
        self.connections.get(&account_id).expect("not found")
    }
}