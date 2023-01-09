use external::{anyhow::Result, async_trait::async_trait, derive_new::new};

use super::RecruitDriver;

#[derive(new)]
pub struct MySQLRecruitDriver {}

#[async_trait]
impl RecruitDriver for MySQLRecruitDriver {
    async fn get_recruits(&self) -> Result<()> {
        todo!()
    }
}
