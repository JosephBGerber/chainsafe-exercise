use crate::ipfs::{Cid, Ipfs};
use crate::store::Store;
use std::fs::File;
use std::path::PathBuf;
use web3::types::Address;

mod ipfs;
mod store;

#[derive(thiserror::Error, Debug)]
pub enum IconError {
    #[error("Failed to read icon from disk.")]
    Io(#[from] std::io::Error),

    #[error("Failed to contact IPFS endpoint.")]
    Ipfs(#[from] ipfs::IpfsError),

    #[error("Failed to contact ethereum endpoint.")]
    Store(#[from] store::StoreError),
}

// Set the icon associated with the given ethereum address to the file at the given path.
pub async fn save_icon(address: Address, path: PathBuf) -> Result<(), IconError> {
    let icon = File::open(path)?;

    let ipfs = Ipfs::new().await?;
    let store = Store::new().await?;

    let cid = ipfs.add_icon(icon).await?;
    store.set_icon(address, cid.into_u256()).await?;
    Ok(())
}

// Get the icon associated with the given ethereum address.
pub async fn get_icon(address: Address) -> Result<Vec<u8>, IconError> {
    let ipfs = Ipfs::new().await?;
    let store = Store::new().await?;

    let ipfs_cid = store.get_icon(address).await?;
    let result = ipfs.get_icon(Cid::from_u256(ipfs_cid)).await?;
    Ok(result)
}
