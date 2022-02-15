use futures::TryStreamExt;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::io::Read;
use web3::types::U256;

#[derive(thiserror::Error, Debug)]
pub enum IpfsError {
    #[error("Failed to contact IPFS endpoint.")]
    Ipfs(#[from] ipfs_api_backend_hyper::Error),

    #[error("Failed to save file to IPFS cluster.")]
    FailedToSave,
}

// CIDv0
#[derive(Debug)]
pub struct Cid([u8; 34]);

impl Cid {
    // Parses a CIDv0 into a CID.
    pub fn from_string(cid: &str) -> Option<Self> {
        if !cid.starts_with("Qm") {
            return None;
        }

        let mut bytes: [u8; 34] = [0; 34];

        bs58::decode(cid).into(&mut bytes).ok()?;

        Some(Cid(bytes))
    }

    // Encodes a CID as a string that can be used to lookup the file contents in IPFS.
    pub fn to_string(&self) -> String {
        bs58::encode(self.0).into_string()
    }

    // Converts a CID to a 256-bit integer by interpreting the bytes of the CID as a big endian integer.
    pub fn into_u256(self) -> U256 {
        U256::from_big_endian(&self.0[2..])
    }

    // Converts a 256-bit integer to a CID by reading the bytes of the integer big endian.
    pub fn from_u256(cid: U256) -> Self {
        let mut bytes: [u8; 34] = [0; 34];
        bytes[0] = 0x12;
        bytes[1] = 0x20;
        let content = &mut bytes[2..];
        cid.to_big_endian(content);
        Cid(bytes)
    }
}

pub struct Ipfs {
    client: IpfsClient,
}

impl Ipfs {
    pub async fn new() -> Result<Self, IpfsError> {
        let client = IpfsClient::default();
        Ok(Ipfs { client })
    }

    pub async fn add_icon<T: 'static + Read + Send + Sync>(
        &self,
        icon: T,
    ) -> Result<Cid, IpfsError> {
        let result = self.client.add(icon).await?;

        let hash = result.hash;

        match Cid::from_string(&hash) {
            Some(cid) => Ok(cid),
            None => Err(IpfsError::FailedToSave),
        }
    }

    pub async fn get_icon(&self, cid: Cid) -> Result<Vec<u8>, IpfsError> {
        let path = cid.to_string();
        let icon = self
            .client
            .cat(&path)
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await?;
        Ok(icon.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::Cid;
    use web3::types::U256;

    #[test]
    fn cid_supports_round_trip_decoding_and_encoding() {
        let cid = "QmarB2kBKuFyEPUhQVjtzMo9f8GTdDuiaPjKAD9camBhn5";
        let decoded = Cid::from_string(cid).unwrap();
        let as_integer: U256 = decoded.into_u256();
        let encoded: Cid = Cid::from_u256(as_integer);

        assert_eq!(&encoded.to_string(), cid);
    }
}
