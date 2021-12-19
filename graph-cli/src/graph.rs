use petgraph::*;
use crate::AccountId;


#[derive(Default, Debug)]
pub struct Connections{
    graph: Graph<AccountId, (u8, u8)>
}

impl Connections{
    pub fn get_graph(&self)->&Graph<AccountId, (u8, u8)>{
        &self.graph
    }
    pub fn get_graph_mut(&mut self)->&mut Graph<AccountId, (u8, u8)>{
        &mut self.graph
    }
}