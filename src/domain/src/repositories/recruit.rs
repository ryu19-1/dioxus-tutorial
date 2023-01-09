use external::{anyhow::Result, async_trait::async_trait, derive_new::new};
use infrastructure::drivers::RecruitDriver;

use super::ReadRecruit;

#[derive(new)]
pub struct ReadRecruitRepository {
    driver: Box<dyn RecruitDriver + Sync + Send>,
}

#[async_trait]
impl ReadRecruit for ReadRecruitRepository {
    async fn read_recruits(&self) -> Result<()> {
        // TODO: ドメインバリデーションなど
        todo!()
    }
}
