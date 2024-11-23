use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC_20,
    "src/utils/contracts/abis/erc20.json"
);
