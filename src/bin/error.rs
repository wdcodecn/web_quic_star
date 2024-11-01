use alloy::eips::eip1898::ParseBlockIdError;
use alloy::eips::eip1898::ParseBlockIdError::FromHexError;
use alloy::hex::FromHexError::OddLength;
use web_quick_start::api_doc::errors::AppError;
use web_quick_start::{set_env, set_log};

fn main() {
    set_env();
    set_log();
    println!("{:?}", err());
    println!("{:?}", err());
}
fn err() -> Result<(), AppError> {
    Ok(err2()?)
}
fn err2() -> Result<(), ParseBlockIdError> {
    Err(FromHexError(OddLength))
}
