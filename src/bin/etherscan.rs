use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Debug, Deserialize)]
struct EtherscanResponse {
    status: String,
    message: String,
    result: String, // 对应余额，通常是字符串格式
}

async fn fetch_balance(
    client: Arc<Client>,
    chain: u32,
    address: &str,
    api_key: &str,
) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://api.etherscan.io/v2/api?chainid={}&module=account&action=balance&address={}&tag=latest&apikey={}",
        chain, address, api_key
    );

    let response: EtherscanResponse = client.get(&url).send().await?.json().await?;

    if response.status == "1" {
        Ok(response.result) // 返回余额
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error: {}", response.message),
        )))
    }
}

#[tokio::main]
async fn main() {
    let client = Arc::new(Client::new());
    let chains = vec![1, 56, 10];
    let address = "0xb5d85cbf7cb3ee0d56b3bb207d5fc4b82f43f511";
    let api_key = "NAXSEKUNWG9VNUYB9TMX4D1IE6IA1QRSUY";

    // 设置并发限制为 5
    let semaphore = Arc::new(Semaphore::new(5));

    let mut handles = Vec::new();

    for chain in chains {
        let client = Arc::clone(&client);
        let semaphore = Arc::clone(&semaphore);
        let address = address.to_string();
        let api_key = api_key.to_string();

        let handle = tokio::spawn(async move {
            // 获取一个许可
            let _permit = semaphore.acquire().await.unwrap();

            // 执行 API 请求
            match fetch_balance(client, chain, &address, &api_key).await {
                Ok(balance) => println!("Chain {}: Balance = {}", chain, balance),
                Err(err) => eprintln!("Error fetching balance for chain {}: {}", chain, err),
            }
        });

        handles.push(handle);
    }

    // 等待所有任务完成
    for handle in handles {
        let _ = handle.await;
    }
}
