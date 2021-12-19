use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};

use petgraph::data::Build;
use petgraph::dot::Dot;
use petgraph::visit::{EdgeRef, IntoNodeIdentifiers, IntoNodeReferences, NodeIndexable};
use petgraph::Graph;
use serde::Deserialize;

mod query;
use query::*;

mod graph;
use graph::*;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    //assert_eq!(args.len(), 1, "Enter your account_id");
    let client = testnet_client();
    
    
    let mut connections = Connections::default();
    let graph = connections.get_graph_mut();
    
    
    let mut unvisited_accounts: HashSet<AccountId> = vec![args.get(0).cloned().unwrap_or("adsick.testnet".to_string())].into_iter().collect();
    let mut visited_accounts = HashSet::new();

    let mut count = 0;
    let limit = 5;

    while count < limit {
        count += 1;
        println!("----");
        println!("{:?}", unvisited_accounts);
        println!("{:?}", visited_accounts);
        println!("----");


        if let Some(account) = unvisited_accounts.iter().find(|a|!visited_accounts.contains(*a)).cloned(){
            println!("fetching unvisited account {}...", account);

            if let Some(unparsed) = query_connections(&client, &account).await{
                println!("unparsed:\n{:?}", unparsed);
                let connections: BTreeMap<String, u8> = serde_json::from_str(&unparsed).expect("parsing error");

                let head =
                if let Some((ix, _)) = graph.node_references().find(|(_, a)|*a == &account){
                    println!("found it on index {:?}", ix);
                    ix
                } else {
                    println!("can't find it, add a new node...");
                    graph.add_node(account.clone())
                };

                println!("added {} to visited list", account);
                visited_accounts.insert(account.clone());
                println!("removed {} from unvisited list", account);
                unvisited_accounts.remove(&account);
                
                for (account, kind) in connections{
                    unvisited_accounts.insert(account.clone());
                    
                    let child =
                    if let Some((ix, _)) = graph.node_references().find(|(_, a)|*a == &account){
                        println!("found it on index {:?}", ix);
                        ix
                    } else {
                        println!("can't find it, add a new node...");
                        graph.add_node(account)
                    };
                    graph.add_edge(head, child, kind);
                }
            } else {
                println!("haven't found any new connections for {}", account);
                unvisited_accounts.remove(&account);
                visited_accounts.insert(account.clone());
            }
        } else {
            println!("no more accounts");
            break;
        }   
    }
    println!("-----------------");
    
    println!("{:?}", graph);
   
    println!("-----------------");

    println!("{:?}", Dot::new(&*graph));
}