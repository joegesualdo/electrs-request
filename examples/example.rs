use bitcoin_utils::{
    self, get_pubkey_hash_from_p2wpkh_address, get_public_key_hash_from_address,
    get_script_hash_from_p2sh_address,
};
use electrs_request::{
    BlockchainRelayFeeCommand, BlockchainScriptHashGetBalanceCommand,
    BlockchainScriptHashGetHistoryCommand, BlockchainScriptHashListUnspentCommand, Client,
};
use hex_utilities::convert_big_endian_hex_to_little_endian;

fn main() {
    let address = "127.0.0.1:50001";
    let client = Client::new(address);

    // let relay_fee_method = "blockchain.relayfee";
    //let relay_fee_response = client.raw_call(relay_fee_method, vec![]).unwrap();
    let response = BlockchainRelayFeeCommand::new().call(&client).unwrap();
    println!("relay fee result: {:?}", response);

    let p2pkh_address = "mw1Bk1AJSs9zaiL5RaQyp1YGfkuruvZAXR".to_string();
    let p2pkh_pk_hash = get_public_key_hash_from_address(&p2pkh_address);
    let p2pkh_script = format!("{}{}{}", "76a914", p2pkh_pk_hash, "88ac");
    let p2pkh_script_sha256 = bitcoin_utils::sha256_hex(&p2pkh_script);
    let p2pkh_script_sha256_le = convert_big_endian_hex_to_little_endian(&p2pkh_script_sha256);
    println!("{}", p2pkh_script_sha256_le);
    let balance_response = BlockchainScriptHashGetBalanceCommand::new(&p2pkh_script_sha256_le)
        .call(&client)
        .unwrap();
    println!("balance: {:#?}", balance_response);
    let unspent_response = BlockchainScriptHashListUnspentCommand::new(&p2pkh_script_sha256_le)
        .call(&client)
        .unwrap();
    println!("unspent: {:#?}", unspent_response);
    let history_response = BlockchainScriptHashGetHistoryCommand::new(&p2pkh_script_sha256_le)
        .call(&client)
        .unwrap();
    println!("history: {:#?}", history_response);

    // let get_balance_method = "blockchain.scripthash.get_balance";
    // let list_unspent_method = "blockchain.scripthash.listunspent";
    // let params = vec![Param::String(p2pkh_script_sha256_le.to_string())];
    // let p2pkh_balance_response = client.raw_call(get_balance_method, params.clone()).unwrap();
    // println!("get balance result: {:#?}", p2pkh_balance_response);
    // let p2pkh_list_unspent_response = client
    //     .raw_call(list_unspent_method, params.clone())
    //     .unwrap();
    // println!("list unspent result: {:#?}", p2pkh_list_unspent_response);

    // let p2sh_address = "2MzBNKyJjx44BDJfwEevVzS3Q9Z5kSEYUZB".to_string();
    // let p2sh_script_hash = get_script_hash_from_p2sh_address(&p2sh_address);
    // let p2sh_script = format!("{}{}{}", "a914", p2sh_script_hash, "87");
    // let p2sh_script_sha256 = bitcoin_utils::sha256_hex(&p2sh_script);
    // let p2sh_script_sha256_le = convert_big_endian_hex_to_little_endian(&p2sh_script_sha256);
    // println!("{}", p2sh_script_sha256_le);

    // let p2wpkh_address = "tb1qphdqqxupe6dzkz3z58twy5l4s0v24mle5gkp99".to_string();
    // let p2wpkh_pk_hash = get_public_key_hash_from_address(&p2wpkh_address);
    // let p2wpkh_script = format!("{}{}", "0014", p2wpkh_pk_hash);
    // let p2wpkh_script_sha256 = bitcoin_utils::sha256_hex(&p2wpkh_script);
    // let p2wpkh_script_sha256_le = convert_big_endian_hex_to_little_endian(&p2wpkh_script_sha256);
    // println!("{}", p2wpkh_script_sha256_le);

    // let p2sh_script =
    //     "22512086e0338f1a93bf5f1a0ca8bb79ae25da839c98d730d2c08c0171f444d283b090".to_string();
    // let shaed = bitcoin_utils::sha256_hex(&p2pkh_script);
    // let shaed_le = convert_big_endian_hex_to_little_endian(&shaed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
