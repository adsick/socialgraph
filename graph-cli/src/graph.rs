use petgraph::*;
use crate::AccountId;


#[derive(Default, Debug)]
pub struct Connections{
    graph: Graph<AccountId, (u8, u8)>
}

impl Connections{

}