use std::collections::HashSet;

use near_jsonrpc_client::methods::tx::TransactionInfo;
use near_jsonrpc_client::{methods, JsonRpcClient, NEAR_TESTNET_RPC_URL};

use near_crypto::SecretKey;
use near_primitives::views::QueryRequest::CallFunction;
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use serde::Deserialize;

type AccountId = String;

#[derive(PartialEq, Eq, Hash, PartialOrd, Deserialize, Debug)]
pub enum RelationKind{
    Knows(AccountId, u8),
    DependsOn(AccountId, u8),
    Follows(AccountId),
    Loves(AccountId, u8)
    // WorksFor(AccountId, u8)
    // Partners(AccountId, u8)
}

#[tokio::main]
async fn main() {
    let testnet_client = JsonRpcClient::connect(NEAR_TESTNET_RPC_URL);
    let view_request = methods::query::RpcQueryRequest {
        block_reference: near_primitives::types::Finality::Final.into(),
        request: {
            CallFunction {
                account_id: "dev-1639808650191-52226736321191".parse().unwrap(),
                method_name: "get_status".to_owned(),
                args: serde_json::to_string(&serde_json::json!(
                    {
                        "account_id": "kek",
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
            println!("got bytes: {:?}", serde_json::from_slice::<HashSet<RelationKind>>(&result.result));
            println!("got string: {:?}", String::from_utf8_lossy(&result.result))
        },
        _ => {}
    }
    // println!("{:?}", status.kind);
    // println!("{:?}", serde_json::to_string(&status));
}
