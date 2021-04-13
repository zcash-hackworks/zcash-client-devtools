
use bip39::{Mnemonic, Language, Seed};
use failure::format_err;
use std::path::Path;

use zcash_primitives::{
    consensus::{
        MainNetwork, Parameters, NetworkUpgrade
    }, 
    transaction::Transaction
};
use zcash_client_backend::{data_api::{DecryptedTransaction, WalletRead, WalletWrite, error::Error}, decrypt_transaction};
use zcash_client_sqlite::WalletDB;

fn main() {
    // TODO: get this path from CLI args
    let network = MainNetwork;
    let db_path = "/home/gmale/kg/work/clones/librustzcash/ZcashSdk_mainnet_Data.db";
    let db_path_tmp = "/home/gmale/kg/work/clones/librustzcash/temp.db";

    let db_data = wallet_db(db_path, network).unwrap();
    let db_tmp = wallet_db(db_path_tmp, network).unwrap();

    let phrase = "chat error pigeon main parade window scene breeze scene frog inherit enforce wise resist rotate van pistol coral tide faint arm elegant velvet anxiety";

    show_seed(phrase);
    let tx = load_tx(&db_data, 25);
    let t = tx.unwrap();
    println!("loaded tx: {:?}", &t);
    println!("tx.vout: {}  tx.vin: {}  tx.shout: {}  tx.shin: {}", t.vout.len(), t.vin.len(), t.shielded_outputs.len(), t.shielded_spends.len());

    let mut db_tmp = db_tmp.get_update_ops().unwrap();
    println!("decrypting transaction into temp DB...");
    decrypt_tx_to(&db_data, &mut db_tmp, &network, 3).unwrap()
}

fn wallet_db<P: Parameters>(db_path: &str, params: P) -> Result<WalletDB<P>, failure::Error> {
    if !Path::new(db_path).exists() {
        Err(format_err!("Path {} did not exist", db_path))
    } else {
        WalletDB::for_path(db_path, params)
        .map_err(|e| format_err!("Error opening wallet database connection: {}", e))
    }
}

// fn fetch_tx<P: Parameters>(db_data: &WalletDB<P>, id_tx: i64) -> Result<Transaction, failure::Error> {
//     let tx = load_tx(&db_data, id_tx);
// }

fn load_tx<P: Parameters>(db_data: &WalletDB<P>, id_tx: i64) -> Result<Transaction, failure::Error> {
    return (&db_data).get_transaction(id_tx).map_err(|_| format_err!("Invalid amount, out of range"));
}

/// Take a transaction out of one db, then decrypt it and store it in another db.
/// This is useful for exercising decrypt code to see what it discovers,
/// without contaminating the original data.
fn decrypt_tx_to<P, W, E, N>(db_src: &WalletDB<P>, db_dest: &mut W, params: &P, id_tx: i64) -> Result<(), E> 
where
    E: From<Error<N>>,
    P: Parameters,
    W: WalletWrite<Error = E>
{
    let tx = &load_tx(&db_src, id_tx).unwrap();
    // Fetch the ExtendedFullViewingKeys we are tracking
    let extfvks = db_src.get_extended_full_viewing_keys().unwrap();

    // Height is block height for mined transactions, and the "mempool height" (chain height + 1)
    // for mempool transactions.
    let height = db_src
        .get_tx_height(tx.txid()).unwrap()
        .or(db_src
            .block_height_extrema().unwrap()
            .map(|(_, max_height)| max_height + 1))
        .or_else(|| params.activation_height(NetworkUpgrade::Sapling))
        .ok_or(Error::SaplingNotActive)?;

    let sapling_outputs = decrypt_transaction(params, height, tx, &extfvks);
    let nullifiers = db_src.get_nullifiers().unwrap();

    if !(sapling_outputs.is_empty() && tx.vout.is_empty()) {
        db_dest.store_decrypted_tx(
            &DecryptedTransaction { tx, sapling_outputs: &sapling_outputs, },
            &nullifiers,
        )?;
    }

    Ok(())
}




// seed things

fn show_seed(phrase: &str) {
    let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    let seed = Seed::new(&mnemonic, "");
    println!("{:X}", seed);
}




// GRPC things

// fn init_grpc() -> CompactTxStreamerClient {
//     let tls = {
//         let mut tls_connector = tls_api_rustls::TlsConnector::builder()?;

//         if tls_api_rustls::TlsConnector::supports_alpn() {
//             tls_connector.set_alpn_protocols(&[b"h2"])?;
//         }

//         let tls_connector = tls_connector.build()?;

//         let tls_connector = Arc::new(tls_connector);
//         ClientTlsOption::Tls(LIGHTWALLETD_HOST.to_owned(), tls_connector)
//     };

//     return grpc::ClientBuilder::new(LIGHTWALLETD_HOST, LIGHTWALLETD_PORT)
//         .explicit_tls(tls)
//         .build()
//         .map(|c| service_grpc::CompactTxStreamerClient::with_client(Arc::new(c)))?;

// }