use std::env;
use std::str::FromStr;

pub mod transaction;

// pub fn get_admin_keypair() -> Keypair {
//     let result = env::var("ADMIN_PK").expect("ADMIN_PK environment variable not specified");
//     Keypair::from_base58_string(&result)
// }