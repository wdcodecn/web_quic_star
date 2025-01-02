//! Example of querying logs from the Ethereum network.

use alloy::eips::BlockNumberOrTag;
use alloy::primitives::{Address, LogData, B256};
use alloy::{
    primitives::{address, b256},
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
};
use eyre::Result;
use std::str::FromStr;
use web_quick::contract::uni_pair_v2::UNI_PAIR_V2;
use web_quick::contract::uni_pair_v3::UNI_PAIR_V3;
use web_quick::UNI_PAIR_V3::UNI_PAIR_V3Events;

// use alloy_rpc_types_eth::log:: *;
#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    // let rpc_url = "https://bsc.merkle.io".parse()?;
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Get logs from the latest block
    let latest_block = provider.get_block_number().await?;

    // Create a filter to get all logs from the latest block.
    let filter = Filter::new()
        // .from_block(latest_block)
        .from_block(BlockNumberOrTag::Number(21534988))
        .to_block(BlockNumberOrTag::Number(21534988));

    // Get all logs from the latest block that match the filter.
    // let logs = provider.get_logs(&filter).await?;

    // for log in logs {
    //     println!("{log:?}");
    // }

    // Get all logs from the latest block that match the transfer event signature/topic.
    // let addr = Address::from_str("123123")
    //     .expect(".env USDT_ADDR");
    // let contract = UNI_PAIR_V2::new(addr, provider);
    // contract
    // v3 Swap
    // #[doc = "Event with signature `Swap(address,address,int256,int256,uint160,uint128,int24)` and selector `0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67`.\n```solidity\nevent Swap(address indexed sender, address indexed recipient, int256 amount0, int256 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick);\n```"]
    let transfer_event_signature =
        b256!("c42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67");

    // You could also use the event name instead of the event signature like so:
    // .event("Transfer(address,address,uint256)")

    // 钱包地址 0xf3de3c0d654fda23dad170f0f320a92172509127
    let t1 = "0x000000000000000000000000e592427a0aece92de3edee1f18e0157c05861564"
        .parse::<B256>()
        .unwrap();

    // 钱包地址 0xf92b7baee951053558499fb281a65573dd667626
    let t2 = "0x000000000000000000000000e7742597b205cd9788af0cdab22d0ae23b988e06"
        .parse::<B256>()
        .unwrap();
    let filter = Filter::new()
        .event_signature(transfer_event_signature)
        .topic1(t1)
        .topic2(t2)
        .from_block(latest_block);
    let logs = provider.get_logs(&filter).await?;

    /*
        Transfer event: Log { inner: Log { address: 0xc12af0c4aa39d3061c56cd3cb19f5e62deeaebde, data: LogData { topics: [0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67, 0x000000000000000000000000e592427a0aece92de3edee1f18e0157c05861564, 0x000000000000000000000000e7742597b205cd9788af0cdab22d0ae23b988e06], data: 0x0000000000000000000000000000000000000000000000000000bbc5619d7d68ffffffffffffffffffffffffffffffffffffffffffffffffffff2404573bac5200000000000000000000000000000000000000011578f5185c8dce5a4df23e000000000000000000000000000000000000000000000000000cbdb219ab3d622b000000000000000000000000000000000000000000000000000000000000064a } }, block_hash: Some(0xf67cb5e2aa7a3f9118603460e6c908d54142bc0564916e93b3c3707cd8021b4b), block_number: Some(21534988), block_timestamp: Some(1735799243), transaction_hash: Some(0xbd00e43f181e9675d0af6ff9a389c46acd4395761fea08fcaf09b5ceae2e3374), transaction_index: Some(123), log_index: Some(338), removed: false }
    */
    for log in logs {
        // let decode = UNI_PAIR_V2::Swap::decode_log_data(&LogData::new());
        println!("Transfer event: {log:?}");
        let x = log.log_decode::<UNI_PAIR_V3::Swap>().unwrap();

        /*
         1: sender 0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45
         2: recipient 0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45
            Data
            amount0 : -464088381
            amount1 : 138710000000000000
            sqrtPriceX96 : 1369382301888861406995229248171377
            liquidity : 9168631159801373322
            tick : 195160
        */
        println!("Transfer event -> address: {:?}", x.inner.address); // pair 地址
        println!("Transfer event -> sender: {:?}", x.inner.sender); //
        println!("Transfer event -> recipient: {:?}", x.inner.recipient);
        println!("Transfer event -> amount0: {:?}", x.inner.amount0);
        println!("Transfer event -> amount1: {:?}", x.inner.amount1);
        println!("Transfer event -> sqrtPriceX96: {:?}", x.inner.sqrtPriceX96);
        println!("Transfer event -> liquidity: {:?}", x.inner.liquidity);
        println!("Transfer event -> tick: {:?}", x.inner.tick);
    }

    // // Get all logs from the latest block emitted by the UNI token address.
    // let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
    // let filter = Filter::new().address(uniswap_token_address).from_block(latest_block);
    //
    // // Get all logs from the latest block that match the filter.
    // let logs = provider.get_logs(&filter).await?;
    //
    // for log in logs {
    //     println!("Uniswap token logs: {log:?}");
    // }

    Ok(())
}
