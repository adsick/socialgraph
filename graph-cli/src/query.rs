use std::borrow::Cow;

use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::methods::tx::TransactionInfo;
use near_jsonrpc_client::{methods, JsonRpcClient, NEAR_TESTNET_RPC_URL};

use near_crypto::SecretKey;
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::views::QueryRequest::CallFunction;
use near_primitives::types::Finality;
pub type AccountId = String;

const SOCIAL_GRAPH_CONTRACT: &str = "sg.adsick.testnet";

pub fn testnet_client()->JsonRpcClient<Unauthenticated>{
    JsonRpcClient::connect(NEAR_TESTNET_RPC_URL)
}

pub async fn query_connections(client: &JsonRpcClient<Unauthenticated>, account_id: &AccountId) -> Option<String> {
    let rpc_query_request = methods::query::RpcQueryRequest {
        block_reference: Finality::Final.into(),
        request: {
            CallFunction {
                account_id: SOCIAL_GRAPH_CONTRACT.parse().unwrap(),
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
    };

    if let Ok(responce) = client.call(rpc_query_request).await{

        if let QueryResponseKind::CallResult(call_result) = responce.kind{
            
            Some(String::from_utf8_lossy(&call_result.result).to_string())
        } else {
            None
        }

    } else {
        None
    }

}