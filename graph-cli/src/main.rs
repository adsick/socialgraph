use std::collections::{BTreeMap, HashSet};

use near_jsonrpc_client::methods::tx::TransactionInfo;
use near_jsonrpc_client::{methods, JsonRpcClient, NEAR_TESTNET_RPC_URL};

use near_crypto::SecretKey;
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::views::QueryRequest::CallFunction;
use petgraph::visit::{EdgeRef, IntoNodeIdentifiers, IntoNodeReferences, NodeIndexable};
use petgraph::Graph;
use serde::Deserialize;

type AccountId = String;

const CONNECT_ACCOUNT_ID: &str = "sg.adsick.testnet";

mod graph;
use graph::*;

fn get_connection_for(account_id: &AccountId) -> methods::query::RpcQueryRequest {
    methods::query::RpcQueryRequest {
        block_reference: near_primitives::types::Finality::Final.into(),
        request: {
            CallFunction {
                account_id: CONNECT_ACCOUNT_ID.parse().unwrap(),
                method_name: "get_connections_for".to_owned(),
                args: serde_json::to_string(&serde_json::json!(
                    {
                        "account_id": account_id,
                    }
                ))
                .unwrap()
                .into_bytes()
                .into(),
            }
        },
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 1, "Enter your account_id");
    let testnet_client = JsonRpcClient::connect(NEAR_TESTNET_RPC_URL);
    
    
    let mut connections = Connections::default();
    let graph = connections.get_graph_mut();
    
    
    let mut account = args[0].clone();

    for i in 0..10{
        let get_connection_for = get_connection_for(&account);
        match testnet_client.call(get_connection_for).await {
            Ok(res) => {
    
                match res.kind {
                    QueryResponseKind::CallResult(result) => {
                        let query_result =
                            serde_json::from_slice::<BTreeMap<AccountId, (u8, u8)>>(&result.result).unwrap();
                        let current_node = graph.add_node(account.clone());
                        for connection in query_result {
                            account = connection.0;
                            let new_node_ix =
                                if let Some((ix, _)) = graph.node_references().find(|n| n.1 == &account) {
                                    ix
                                } else {
                                    graph.add_node(account.clone())
                                };
                                
                            graph.add_edge(current_node, new_node_ix, connection.1);
                        }
                        println!("got string: {:?}", String::from_utf8_lossy(&result.result))
                    }
                    _ => {}
                }
    
    
    
            },
            Err(_) => {
                break;
            },
        };

    }
    println!("-----------------");

    println!("{:?}", graph);


    


    
}
