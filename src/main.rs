
use bip39::{Mnemonic, Language, Seed};
use failure::format_err;
use std::path::Path;

use zcash_primitives::{consensus::{MainNetwork, Parameters}, transaction::Transaction};
use zcash_client_backend::data_api::WalletRead;
use zcash_client_sqlite::WalletDB;

fn main() {
    // TODO: get this path from CLI args
    let db_path = "/home/gmale/kg/work/clones/librustzcash/ZcashSdk_mainnet_Data.db";
    let db_data = wallet_db(db_path, MainNetwork).unwrap();

    let phrase = "chat error pigeon main parade window scene breeze scene frog inherit enforce wise resist rotate van pistol coral tide faint arm elegant velvet anxiety";

    show_seed(phrase);
    let tx = load_tx(&db_data, 3);
    println!("loaded tx: {:?}", &tx.unwrap());
}

fn wallet_db<P: Parameters>(db_path: &str, params: P) -> Result<WalletDB<P>, failure::Error> {
    if !Path::new(db_path).exists() {
        Err(format_err!("Path {} did not exist", db_path))
    } else {
        WalletDB::for_path(db_path, params)
        .map_err(|e| format_err!("Error opening wallet database connection: {}", e))
    }
}

fn load_tx(db_data: &WalletDB<MainNetwork>, id_tx: i64) -> Result<Transaction, failure::Error> {
    return (&db_data).get_transaction(id_tx).map_err(|_| format_err!("Invalid amount, out of range"));
}





// seed things

fn show_seed(phrase: &str) {
    let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    let seed = Seed::new(&mnemonic, "");
    println!("{:X}", seed);
}