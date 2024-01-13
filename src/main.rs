// use bdk::{Wallet,SyncOptions};
// use bdk::database::MemoryDatabase;
// use bdk::blockchain::ElectricBlockChain;
// use bdk::electrum_client::Client;
use std::env;
use dotenv::from_filename;
fn main() {
    

    from_filename(".env").ok();

    let descriptor = env::var("WALLET_DESCRIPTOR").unwrap();

    println!("Hello, world!");
    println!("The value of env is {}", descriptor);

    dbg!(descriptor);
}

