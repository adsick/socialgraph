use std::collections::{HashSet, BTreeMap};

use near_jsonrpc_client::methods::tx::TransactionInfo;
use near_jsonrpc_client::{methods, JsonRpcClient, NEAR_TESTNET_RPC_URL};

use near_crypto::SecretKey;
use near_primitives::views::QueryRequest::CallFunction;
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use serde::Deserialize;

type AccountId = String;

mod graph;
use graph::*;

fn get_connection_for(account_id: &AccountId) -> methods::query::RpcQueryRequest {
    methods::query::RpcQueryRequest {
        block_reference: near_primitives::types::Finality::Final.into(),
        request: {
            CallFunction {
                account_id: "sg.adsick.testnet".parse().unwrap(),
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
    let get_connection_for = get_connection_for(&args[0]);
    let status = testnet_client.call(get_connection_for).await.unwrap();
    match status.kind{
        QueryResponseKind::CallResult(result) => {
            println!("got bytes: {:?}", serde_json::from_slice::<BTreeMap<AccountId, (u8, u8)>>(&result.result));
            println!("got string: {:?}", String::from_utf8_lossy(&result.result))
        },
        _ => {}
    }

    let mut connections = Connections::default();

    


}
