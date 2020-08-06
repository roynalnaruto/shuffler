use anyhow::anyhow;
use ethers::prelude::*;
use std::{env, str::FromStr, time::Duration};

mod bindings;
use bindings::IERC20;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (url, private_key) = env_var()?;
    let ws = Ws::connect(url).await?;
    let provider = Provider::new(ws).interval(Duration::from_secs(5));
    let client = private_key.parse::<Wallet>()?.connect(provider);

    // mainnet USDT: https://etherscan.io/address/0xdac17f958d2ee523a2206206994597c13d831ec7
    let address = Address::from_str("dAC17F958D2ee523a2206206994597C13D831ec7")?;
    let contract = IERC20::new(address, client);

    let mut stream = contract.transfer_filter().stream().await?;

    while let Some(item) = stream.next().await {
        match item {
            Ok(log) => {
                dbg!("[from]  : {:?}", log.from);
                dbg!("[to]    : {:?}", log.to);
                dbg!("[value] : {:?}", log.value);
            }
            Err(e) => {
                dbg!("[error] : {:?}", e);
            }
        }
    }

    Ok(())
}

fn env_var() -> anyhow::Result<(String, String)> {
    let url = match env::var("PROVIDER_WSS_URL") {
        Ok(url) => url,
        Err(e) => return Err(anyhow!("Missing env var: {}", e)),
    };

    let private_key = match env::var("PRIVATE_KEY") {
        Ok(pk) => pk,
        Err(e) => return Err(anyhow!("Missing env var: {}", e)),
    };

    Ok((url, private_key))
}
