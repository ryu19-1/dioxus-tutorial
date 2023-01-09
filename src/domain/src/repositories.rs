use external::{anyhow::Result, async_trait::async_trait};

pub mod recruit;

#[async_trait]
pub trait ReadRecruit {
    async fn read_recruits(&self) -> Result<()>;
}
