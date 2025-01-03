
/// ETH
pub const UNISWAP_V3_FACTORY_ETH: &str = "0x1F98431c8aD98523631AE4a59f267346ea31F984"; // Ethereum上的Uniswap V3 Factory
pub const SWAP_ROUTER02_ETH: &str = "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45"; // Ethereum上的SwapRouter02
pub const SWAP_ROUTER_ETH: &str = "0xE592427A0AEce92De3Edee1F18E0157C05861564"; // Ethereum上的SwapRouter

/// BSC
pub const UNISWAP_V3_FACTORY_BSC: &str = "0xdB1d10011AD0Ff90774D0C6Bb92e5C5c8b4461F7"; // BSC上的Uniswap V3 Factory
pub const SWAP_ROUTER02_BSC: &str = "0xB971eF87ede563556b2ED4b1C0b0019111Dd85d2"; // BSC上的SwapRouter02

/// BASE
pub const UNISWAP_V3_FACTORY_BASE: &str = "0x33128a8fC17869897dcE68Ed026d694621f6FDfD"; // BASEereum上的Uniswap V3 Factory
pub const SWAP_ROUTER02_BASE: &str = "0x2626664c2603336E57B271c5C0b26F421741e481"; // BASEereum上的SwapRouter02


/// Etherscan API Key
pub const ETHERSCAN_API_KEY :&str = "NAXSEKUNWG9VNUYB9TMX4D1IE6IA1QRSUY";
pub const BSCSCAN_API_KEY :&str = "";
pub const BASESCAN_API_KEY :&str = "";

pub enum Chain {
    Eth,
    Bsc,
    Base,
}

pub struct UniswapAddresses {
    pub factory: &'static str,
    pub swap_router: &'static str,
    pub api_key: &'static str,
}

impl UniswapAddresses {
    pub fn from_chain(chain: Chain) -> Self {
        match chain {
            Chain::Eth => UniswapAddresses {
                factory: UNISWAP_V3_FACTORY_ETH,
                swap_router: SWAP_ROUTER02_ETH,
                api_key: ETHERSCAN_API_KEY,
            },
            Chain::Bsc => UniswapAddresses {
                factory: UNISWAP_V3_FACTORY_BSC,
                swap_router: SWAP_ROUTER02_BSC,
                api_key: BSCSCAN_API_KEY,
            },
            Chain::Base => UniswapAddresses {
                factory: UNISWAP_V3_FACTORY_BASE,
                swap_router: SWAP_ROUTER02_BASE,
                api_key: BASESCAN_API_KEY,
            },
        }
    }
}
