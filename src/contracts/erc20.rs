use alloy::hex::FromHex;
use alloy::sol;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use std::str::FromStr;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC_20,
    "src/contracts/abis/erc20.json"
);
