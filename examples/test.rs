extern crate ethrpc;

use ethrpc::{HttpRpc, RpcRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let rpc = HttpRpc::new("https://mainnet.infura.io/mew")?;
    let id = rpc.prepare_request();

    let request = RpcRequest::new(id, "eth_blockNumber", vec![]);

    rpc.send(request).await?;

    Ok(())
}
