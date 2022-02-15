use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web3::contract::{Contract, Options};
use web3::ethabi;
use web3::transports::Http;
use web3::types::{Address, H160, U256};

#[derive(thiserror::Error, Debug)]
pub enum StoreError {
    #[error("Failed to contact ethereum endpoint.")]
    Ethereum(#[from] web3::Error),

    #[error("Failed to execute method on contract.")]
    Contract(#[from] web3::contract::Error),
}

pub struct Store {
    contract: Contract<Http>,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    abi: ethabi::Contract,
    networks: HashMap<String, Network>,
}

#[derive(Serialize, Deserialize)]
struct Network {
    address: String,
}

impl Store {
    pub async fn new() -> Result<Self, StoreError> {
        let transport = web3::transports::Http::new("http://localhost:7545")?;
        let web3 = web3::Web3::new(transport);
        let metadata: Metadata =
            serde_json::from_slice(include_bytes!("../../ethereum/build/contracts/Icon.json"))
                .expect("Invalid contract JSON when compiled.");

        let network_id = web3.net().version().await?;

        match metadata.networks.get(&network_id) {
            Some(network) => {
                let address: Address = H160::from_slice(
                    &hex::decode(&network.address[2..])
                        .expect("Invalid network address in contract JSON."),
                );
                let contract = Contract::new(web3.eth(), address, metadata.abi);
                Ok(Store { contract })
            }
            None => {
                panic!("The Main contract is not deployed on the given instance.")
            }
        }
    }

    pub async fn set_icon(
        &self,
        ethereum_address: Address,
        ipfs_cid: U256,
    ) -> Result<(), StoreError> {
        self.contract
            .call("setIcon", ipfs_cid, ethereum_address, Options::default())
            .await?;
        Ok(())
    }

    pub async fn get_icon(&self, ethereum_address: Address) -> Result<U256, StoreError> {
        let icon: U256 = self
            .contract
            .query("getIcon", ethereum_address, None, Options::default(), None)
            .await?;
        Ok(icon)
    }
}
