use external::{anyhow::Result, async_trait::async_trait};

pub mod get_recruits;

#[async_trait]
pub trait GetRecruits {
    async fn handle(&self) -> Result<()>;
}
