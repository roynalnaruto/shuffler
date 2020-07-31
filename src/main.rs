use anyhow::anyhow;
use ethers::prelude::*;
use std::{env, str::FromStr, time::Duration};

mod bindings;
use bindings::BeaconContract;

mod list;
use list::RandomisableList;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rand_list = RandomisableList::new(10);
    dbg!("[list]: {:?}", rand_list.list());

    let (url, private_key) = env_var()?;
    let ws = Ws::connect(url).await?;
    let provider = Provider::new(ws).interval(Duration::from_secs(5));
    let client = private_key.parse::<Wallet>()?.connect(provider);

    let address = Address::from_str("79474439753C7c70011C3b00e06e559378bAD040")?;
    let beacon_contract = BeaconContract::new(address, client);

    let mut stream = beacon_contract.log_new_randomness_filter().stream().await?;

    while let Some(item) = stream.next().await {
        match item {
            Ok(log) => {
                dbg!("[block]: {:?}", log.block_number);
                rand_list.shuffle(log.randomness);
                dbg!("[list]: {:?}", rand_list.list());
            }
            Err(e) => {
                dbg!("[error]: {:?}", e);
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
