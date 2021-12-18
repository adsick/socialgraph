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

#[tokio::main]
async fn main() {
    let testnet_client = JsonRpcClient::connect(NEAR_TESTNET_RPC_URL);
    let view_request = methods::query::RpcQueryRequest {
        block_reference: near_primitives::types::Finality::Final.into(),
        request: {
            CallFunction {
                account_id: "sg.adsick.testnet".parse().unwrap(),
                method_name: "get_connections_for".to_owned(),
                args: serde_json::to_string(&serde_json::json!(
                    {
                        "account_id": "adsick.testnet",
                    }
                ))
                .unwrap()
                .into_bytes()
                .into(),
            }
        },
    };
    let status = testnet_client.call(view_request).await.unwrap();
    match status.kind{
        QueryResponseKind::CallResult(result) => {
            println!("got bytes: {:?}", serde_json::from_slice::<BTreeMap<AccountId, (u8, u8)>>(&result.result));
            println!("got string: {:?}", String::from_utf8_lossy(&result.result))
        },
        _ => {}
    }

    let mut connections = Connections::default();

    


}
