use external::{async_trait::async_trait, anyhow::Result};

pub mod recruit;

#[async_trait]
pub trait RecruitDriver {
    async fn get_recruits(&self) -> Result<()>;
}