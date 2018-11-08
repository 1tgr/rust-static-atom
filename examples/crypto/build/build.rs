extern crate static_atom;

use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::result::Result;

use static_atom::build;

fn main() -> Result<(), Box<Error>> {
    let filename = Path::new(&env::var("OUT_DIR")?).join("atoms.rs");
    let mut file = File::create(filename)?;

    build::generate(&mut file, "Small", vec!["BTC-EUR", "BTC-USDC", "ETH-EUR", "ETH-BTC"])?;

    build::generate(
        &mut file,
        "Big",
        vec![
            "BTC-EUR", "BTC-USDC", "ETH-EUR", "ETH-BTC", "ETH-USDC", "ETC-BTC", "ETC-EUR", "BTC-USD", "BCH-BTC",
            "BCH-USD", "BTC-GBP", "ETH-USD", "LTC-BTC", "LTC-EUR", "LTC-USD", "BCH-EUR", "ETC-USD", "ZRX-USD",
            "ZRX-BTC", "ZRX-EUR", "ETC-GBP", "ETH-GBP", "LTC-GBP", "BCH-GBP",
        ],
    )?;
    Ok(())
}
