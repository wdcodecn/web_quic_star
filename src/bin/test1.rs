use alloy::rpc::types::Log;
use alloy::sol_types::SolEvent;
use alloy_chains::{Chain, ChainKind, NamedChain};
use alloy_primitives::bytes::Bytes;
use alloy_primitives::{Address, LogData, B256};
use chrono::{Duration, Utc};
use foundry_block_explorers::account::{AccountBalance, NormalTransaction, Sort, TxListParams};
use foundry_block_explorers::blocks::BlockNumberByTimestamp;
use foundry_block_explorers::Client;
use reqwest::Url;
use serde::de::Error;
use std::str::FromStr;
use web_quick::constant::{BASESCAN_API_KEY, BSCSCAN_API_KEY, ETHERSCAN_API_KEY};
use web_quick::UNI_PAIR_V3::Swap;
use web_quick::{set_env, UNI_PAIR_V3};

async fn eth() -> Option<AccountBalance> {
    let client = Client::new(Chain::mainnet(), ETHERSCAN_API_KEY).unwrap();
    // let client = Client::new_from_env(Chain::mainnet())?;

    let address = "0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413"
        .parse()
        .unwrap();
    get_ether_balance_single(client, &address).await
}

async fn bsc() -> Option<AccountBalance> {
    let client = Client::new(Chain::bsc_mainnet(), "NAXSEKUNWG9VNUYB9TMX4D1IE6IA1QRSUY").unwrap();
    // Or using environment variables
    // let client = Client::new_from_env(Chain::bsc_mainnet())?;

    let address = "0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413"
        .parse()
        .unwrap();
    get_ether_balance_single(client, &address).await
}

async fn base() -> Option<AccountBalance> {
    let client = Client::new(Chain::base_mainnet(), "NAXSEKUNWG9VNUYB9TMX4D1IE6IA1QRSUY").unwrap();
    // let client = Client::new_from_env(Chain::base_mainnet())?;

    let address = "0x58eB28A67731c570Ef827C365c89B5751F9E6b0a"
        .parse()
        .unwrap();

    get_ether_balance_single(client, &address).await
}

async fn get_ether_balance_single(client: Client, address: &Address) -> Option<AccountBalance> {
    client.get_ether_balance_single(&address, None).await.ok()
}

async fn get_transactions(client: Client, address: &Address) -> Option<Vec<NormalTransaction>> {
    client
        .get_transactions(
            &address,
            Some(TxListParams {
                start_block: 0,
                end_block: 99999999,
                page: 0,
                offset: 10,
                sort: Sort::Asc,
            }),
        )
        .await
        .ok()
}

pub fn get_scan_client(chain: Chain) -> Client {
    match chain.named().unwrap() {
        NamedChain::Mainnet => Client::new(chain, ETHERSCAN_API_KEY).unwrap(),
        NamedChain::BinanceSmartChain => Client::new(chain, BSCSCAN_API_KEY).unwrap(),
        NamedChain::Base => Client::new(chain, BASESCAN_API_KEY).unwrap(),
        _ => {
            panic!("Unsupported chain")
        }
    }
}
async fn calculate_one_year_ago_block(current_block: u64) -> u64 {
    const BLOCKS_PER_YEAR: u64 = 2_592_000; // 每年约 259.2 万个区块
    if current_block > BLOCKS_PER_YEAR {
        current_block - BLOCKS_PER_YEAR
    } else {
        0 // 如果当前区块号小于一年内的总区块数
    }
}

#[tokio::main]
async fn main() {
    web_quick::set_log();
    set_env();

    let eth_address: Address = "0xd4a2AEE94345BfA6Aa1BdD3B95E8dc9d14b2eA19"
        .parse()
        .unwrap();
    let bsc_address: Address = "0xB79D4F62B3c3dB98B486F91A102792b3Af006Dec"
        .parse()
        .unwrap();
    let base_address: Address = "0x1bd48bcb8fcc89bbfa4ff5d07021e5d673f480ef"
        .parse()
        .unwrap();

    let mainnet = start_filter_log(Chain::mainnet(), &eth_address).await;
    // let bsc_mainnet = start_filter_log(Chain::bsc_mainnet(), &bsc_address);
    // let base_mainnet = start_filter_log(Chain::base_mainnet(), &base_address);
    //
    // tokio::join!(mainnet, bsc_mainnet, base_mainnet);
}

pub async fn start_filter_log(
    chain: Chain,
    address: &Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_scan_client(chain);

    let now = Utc::now();
    // 一年前的时间
    let one_year_ago = now - Duration::days(365);

    // 时间戳 (秒级别)
    let one_year_ago_timestamp = one_year_ago.timestamp();

    /// 获取一年前的最近的区块号
    let block_number_after: BlockNumberByTimestamp = client
        .get_block_by_timestamp(one_year_ago_timestamp as u64, "after")
        .await
        .ok()
        .unwrap();

    /// 获取一年前的最近的区块号
    let start_block = block_number_after
        .block_number
        .as_number()
        .unwrap()
        .to_string()
        .parse::<u64>()
        .unwrap_or(0);

    println!("{:?}", start_block);

    /// 获取用户的交易列表 取第一笔交易 区块号从 start_block 开始
    let transactions = client
        .get_transactions(
            address,
            Some(TxListParams {
                start_block: start_block,
                end_block: 99999999,
                page: 1,
                offset: 1,
                sort: Sort::Asc,
            }),
        )
        .await
        .ok();
    match transactions {
        None => {}
        Some(trxs) => {
            if trxs.is_empty() {
                println!("没有交易记录");
            } else {
                let first_transaction = trxs.first().unwrap();
                println!("{:?}", first_transaction);

                let start_block = first_transaction
                    .block_number
                    .as_number()
                    .unwrap()
                    .to_string()
                    .parse::<u64>()
                    .unwrap_or(0);

                let address = &format!("{address:?}");
                let topic0 = "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67";
                let topic1 = address;
                let topic2 = address;
                let topic1_2_opr = "or";

                /// 获取logs
                let result = client
                    .get_logs(
                        21541017,
                        99999999,
                        "0x2a8Ff49323d42Be0Af7a1b2f3005b67da55E0c6b",
                        topic0,
                        topic1,
                        topic2,
                    )
                    .await
                    .ok();

                println!("{:?}", result);
            }
        }
    }

    Ok(())
}

fn pad_address(address: &str) -> Result<String, &'static str> {
    if !address.starts_with("0x") || address.len() != 42 {
        return Err("Invalid Ethereum address format");
    }

    let stripped = &address[2..]; // 去掉 "0x" 前缀
    let padded = format!("{:0>64}", stripped); // 补齐到 64 个字符
    Ok(format!("0x{}", padded))
}

#[tokio::test]
async fn test_get_logs() {
    web_quick::set_log();
    set_env();
    let client = get_scan_client(Chain::mainnet());

    let uniswap_address: &str = "0x2a8Ff49323d42Be0Af7a1b2f3005b67da55E0c6b";
    let user_address: &str = "0xd4a2AEE94345BfA6Aa1BdD3B95E8dc9d14b2eA19";
    let topic0 = "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67";

    // println!("{:?}", pad_address(user_address));

    let binding = pad_address(user_address).unwrap();
    let topic1 = binding.as_str();
    println!("{:?}", topic1);
    let binding1 = pad_address(user_address).unwrap();
    let topic2 = binding1.as_str();
    println!("{:?}", topic2);

    /// 获取logs
    let result = client
        .get_logs(21541017, 99999999, "", topic0, topic1, topic2)
        .await;
    println!("{:?}", result);
    for log_entry in result.unwrap() {
        let topics: Vec<B256> = log_entry
            .topics
            .iter()
            .map(|x| x.parse::<B256>().unwrap())
            .collect::<Vec<B256>>();

        let logdata = LogData::new(
            topics,
            Bytes::from(log_entry.data.clone().into_bytes()).into(),
        )
        .unwrap();
        let decoded: UNI_PAIR_V3::Swap =
            UNI_PAIR_V3::Swap::decode_log_data(&logdata, false).unwrap();
        println!("pair address -> {:?}", log_entry.address);
        println!("sender -> {:?}", decoded.sender);
        println!("recipient -> {:?}", decoded.recipient);
        println!("amount0 -> {:?}", decoded.amount0);
        println!("amount1 -> {:?}", decoded.amount1);
        println!("liquidity -> {:?}", decoded.liquidity);
        println!("sqrtPriceX96 -> {:?}", decoded.sqrtPriceX96);

        // 根据pair 获取 token0 token1
        // let pair_address = pad_address(log_entry.address.as_str()).unwrap();
        let pair_address =  log_entry.address.as_str();

        let url = Url::from_str(std::env::var("ETH_RPC").expect(".env ETH_RPC").as_str()).unwrap();

        let provider = alloy::providers::ProviderBuilder::new()
            .with_recommended_fillers()
            .on_http(url);

        println!("pair_address -> {:?}", pair_address);

        let pair_address = Address::from_str(pair_address).unwrap();

        println!("pair_address -> {:?}", pair_address);

        let uni_pair_v3 =
            UNI_PAIR_V3::new(pair_address, provider);
        let token0 = uni_pair_v3.token0().call().await.unwrap();
        let token1 = uni_pair_v3.token1().call().await.unwrap();
        println!("token0 -> {:?}", token0._0.to_string());
        println!("token1 -> {:?}", token1._0.to_string());

        // if decoded.sender.to_string().eq(user_address) {
        //     // 卖出代币
        // }
        // if decoded.sender.to_string().eq(user_address) {
        //     // 卖出代币
        // }
    }
    // Ok(Log {
    //     inner: decoded,
    //     block_hash: self.block_hash,
    //     block_number: self.block_number,
    //     block_timestamp: self.block_timestamp,
    //     transaction_hash: self.transaction_hash,
    //     transaction_index: self.transaction_index,
    //     log_index: self.log_index,
    //     removed: self.removed,
    // })
}
