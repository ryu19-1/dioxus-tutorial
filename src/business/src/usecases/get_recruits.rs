use domain::repositories::ReadRecruit;
use external::{anyhow::Result, async_trait::async_trait, derive_new::new};

use super::GetRecruits;

#[derive(new)]
pub struct GetRecruitsUseCase {
    repository: Box<dyn ReadRecruit + Sync + Send>,
}

#[async_trait]
impl GetRecruits for GetRecruitsUseCase {
    async fn handle(&self) -> Result<()> {
        todo!()
    }
}
