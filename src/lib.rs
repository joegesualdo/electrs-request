use electrum_client::{
    raw_client::RawClient, Client as ElectrumClient, Config, ElectrumApi, Param,
};
use serde::{Deserialize, Serialize};

pub struct Client {
    electrum_client: ElectrumClient,
}
impl Client {
    pub fn new(electrum_server_address: &str) -> Self {
        let address = format!("tcp://{}", electrum_server_address);
        let config = Config::default();
        let client = ElectrumClient::from_config(&address, config).unwrap();
        Client {
            electrum_client: client,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainRelayFeeCommandResponse(f64);

pub struct BlockchainRelayFeeCommand {}
impl BlockchainRelayFeeCommand {
    pub fn new() -> Self {
        BlockchainRelayFeeCommand {}
    }
    pub fn call(
        &self,
        client: &Client,
    ) -> Result<BlockchainRelayFeeCommandResponse, electrum_client::Error> {
        let relay_fee_method = "blockchain.relayfee";
        let relay_fee_response_result =
            client.electrum_client.raw_call(relay_fee_method, vec![])?;
        let blockchain_relay_fee_command_response: BlockchainRelayFeeCommandResponse =
            serde_json::from_value(relay_fee_response_result).unwrap();
        Ok(blockchain_relay_fee_command_response)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainScriptHashGetBalanceCommand {
    pub script_hash: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainScriptHashGetBalanceCommandResponse {
    pub confirmed: u64,
    pub unconfirmed: u64,
}
impl BlockchainScriptHashGetBalanceCommand {
    pub fn new(script_hash: &str) -> Self {
        BlockchainScriptHashGetBalanceCommand {
            script_hash: script_hash.to_string(),
        }
    }
    pub fn call(
        &self,
        client: &Client,
    ) -> Result<BlockchainScriptHashGetBalanceCommandResponse, electrum_client::Error> {
        let get_balance_method = "blockchain.scripthash.get_balance";
        let params = vec![Param::String(self.script_hash.to_owned())];
        let get_balance_response_result = client
            .electrum_client
            .raw_call(get_balance_method, params)?;
        let get_balance_command_response: BlockchainScriptHashGetBalanceCommandResponse =
            serde_json::from_value(get_balance_response_result).unwrap();
        Ok(get_balance_command_response)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainScriptHashListUnspentCommand {
    pub script_hash: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UnspentResponse {
    pub height: u64,
    pub tx_hash: String,
    pub tx_pos: u64,
    pub value: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainScriptHashListUnspentCommandResponse(Vec<UnspentResponse>);
impl BlockchainScriptHashListUnspentCommand {
    pub fn new(script_hash: &str) -> Self {
        BlockchainScriptHashListUnspentCommand {
            script_hash: script_hash.to_string(),
        }
    }
    pub fn call(
        &self,
        client: &Client,
    ) -> Result<BlockchainScriptHashListUnspentCommandResponse, electrum_client::Error> {
        let list_unspent_method = "blockchain.scripthash.listunspent";
        let params = vec![Param::String(self.script_hash.to_owned())];
        let list_unspent_response_result = client
            .electrum_client
            .raw_call(list_unspent_method, params)?;
        let list_unspent_command_response: BlockchainScriptHashListUnspentCommandResponse =
            serde_json::from_value(list_unspent_response_result).unwrap();
        Ok(list_unspent_command_response)
    }
}
