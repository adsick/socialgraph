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
    Follows(AccountId),
    Loves(AccountId, u8)
    // WorksFor(AccountId, u8)
    // Partners(AccountId, u8)
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn know(&mut self, mate: AccountId, distance: u8){
        let predecessor = env::predecessor_account_id();
        self.add_relation(predecessor, RelationKind::Knows(mate, distance))
    }

    pub fn depend_on(&mut self, account_id: AccountId, distance: u8){
        let predecessor = env::predecessor_account_id();
        self.add_relation(predecessor, RelationKind::DependsOn(account_id, distance))
    }

    fn add_relation(&mut self, account_id: AccountId, relation: RelationKind){
        let mut relations = self.relations.get(&account_id).unwrap_or_default();
        relations.insert(relation);
        self.relations.insert(&account_id, &relations);
    }

    pub fn get_relations(&self, account_id: AccountId)->HashSet<RelationKind>{
        self.relations.get(&account_id).expect("no relations")
    }
}
