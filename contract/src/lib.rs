use std::collections::HashSet;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, AccountId, env};



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    relations: UnorderedMap<AccountId, HashSet<RelationKind>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            relations: UnorderedMap::new(b"r"),
        }
    }
}

// note, you can use u8 bitmap to express 8 kinds of relations
// but we stick to enum for now

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Eq, Hash, PartialOrd)]
pub enum RelationKind{
    Knows(AccountId, u8),
    DependsOn(AccountId, u8),
    // WorksFor(AccountId, u8)
    // Partners(AccountId, u8)
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn trust_to(&mut self, mate: AccountId, distance: u8){
        let predecessor = env::predecessor_account_id();
        let mut relation_set = self.relations.get(&predecessor).unwrap_or_default();
        relation_set.insert(RelationKind::Knows(mate, distance));
        self.relations.insert(&predecessor, &relation_set);
    }

    pub fn get_relations(&self, account_id: AccountId)->HashSet<RelationKind>{
        self.relations.get(&account_id).expect("no relations")
    }
}
