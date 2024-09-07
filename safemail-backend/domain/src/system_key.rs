use crate::error::SmError;
use async_trait::async_trait;

pub struct SystemKeyPair {
    pub private_key: String,
    pub public_key: String,
}

#[async_trait]
pub trait SystemKeyRepository {
    async fn init_system_keys(&self, system_key: SystemKeyPair) -> Result<(), SmError>;
    async fn get_system_keys(&self) -> Result<Option<SystemKeyPair>, SmError>;
}
