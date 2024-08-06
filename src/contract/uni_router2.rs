//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

// use alloy::{node_bindings::Anvil, primitives::address, providers::ProviderBuilder, sol};
// use eyre::Result;

use std::env;
use std::error::Error;
use std::str::FromStr;

use alloy::primitives::Address;
use alloy::providers::{ProviderBuilder, ReqwestProvider};
use alloy::sol;
use alloy::transports::http::{Client, Http};
use alloy::transports::http::reqwest::Url;

use crate::contract::uni_router2::UNI_ROUTER2::UNI_ROUTER2Instance;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UNI_ROUTER2,
    "src/contract/uni_router2.json"
);

pub async fn get_uni_router2() -> Result<UNI_ROUTER2Instance<Http<Client>,ReqwestProvider>,Box<dyn Error>> {

  let provider = ProviderBuilder::new().on_http(Url::from_str(env::var("ETH_RPC")?.as_str()).unwrap());

  let uni_router2 = UNI_ROUTER2::new(Address::from_str(env::var("UNI_ROUTER2_ADDR")?.as_str())?, provider);
  
  Ok(uni_router2)
}