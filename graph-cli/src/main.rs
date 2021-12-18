use near_jsonrpc_client::methods::tx::TransactionInfo;
use near_jsonrpc_client::{methods, JsonRpcClient, NEAR_TESTNET_RPC_URL};

use near_crypto::SecretKey;
use near_primitives::views::QueryRequest::CallFunction;

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
    println!("{:?}", serde_json::to_string(&status));
}
