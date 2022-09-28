# Electrs Request 
> Send RPC commands to an Electrs server.

---

**⚠️ This is experimental. Please use at your own risk.⚠️**

---

This library provides typesafe functions over raw RPC Electrs commands to easily and safely retrieve information from a the server node.

As this library only provides typesafety over raw RPC commands, functions will be mapped 1:1 to RPC commands. See [ElectrumX](https://electrumx.readthedocs.io/en/latest/protocol-methods.html#blockchain-estimatefee) for a list of all commands and what they return.

If you're looking for a higher level api for querying information from electrum (i.e. balance for address, etc) , take a look at [electrs-query](https://github.com/joegesualdo/electrs-query), which provides higher-level functions to query an electrs server.

## Install
> Add package to Cargo.toml file
```rust
[dependencies]
electrs-request = "0.1.1"
```

## Usage:
```rust
let electrs_server_address = "127.0.0.1:50001";
let client = Client::new(electrs_server_address);

// get the relay fee
let response = BlockchainRelayFeeCommand::new().call(&client).unwrap();
println!("relay fee result: {:?}", response);

let p2pkh_address = "mv7RvNNQ7HpQf2diQai5hgpeuzkFoAQP9G".to_string();

// And address must be converted to a public key hash, then hashed using sha256 and then converted to little endian before requesting information from electrs.
let p2pkh_pk_hash = get_public_key_hash_from_address(&p2pkh_address);
let p2pkh_script = format!("{}{}{}", "76a914", p2pkh_pk_hash, "88ac");
let p2pkh_script_sha256 = bitcoin_utils::sha256_hex(&p2pkh_script);
let p2pkh_script_sha256_le = convert_big_endian_hex_to_little_endian(&p2pkh_script_sha256);

// get the balance for a p2pkh address
let balance_response = BlockchainScriptHashGetBalanceCommand::new(&p2pkh_script_sha256_le)
.call(&client)
.unwrap();
println!("balance: {:#?}", balance_response);
// get utxos for address
let unspent_response = BlockchainScriptHashListUnspentCommand::new(&p2pkh_script_sha256_le)
.call(&client)
.unwrap();
println!("unspent: {:#?}", unspent_response);

```
## Commands
List of all electrs commands can be found on [exectrumx docs](https://electrumx.readthedocs.io/en/latest/protocol-methods.html#blockchain-estimatefee)

## Related
- [electrs-query](https://github.com/joegesualdo/electrs-query) - Query Electrs server for information
- [bitcoind-request](https://github.com/joegesualdo/bitcoind-request) - Easily send rpc commands to Bitcoin core
- [bitcoin-node-query](https://github.com/joegesualdo/bitcoin-node-query) - Query Bitcoin Node for information
- [bitcoin-terminal-dashboard](https://github.com/joegesualdo/bitcoin-terminal-dashboard) - Bitcoin Dashboard in the terminal

## License
MIT © [Joe Gesualdo]()
