extern crate reth;

use reth::{HttpRpc, RpcRequest};
use std::sync::atomic;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let rpc = HttpRpc::new("https://mainnet.infura.io/mew")?;
    let id = rpc.id.fetch_add(1, atomic::Ordering::Relaxed);

    let request = RpcRequest::new(id, "eth_blockNumber", vec![]);

    rpc.send(request).await?;

    Ok(())
}
