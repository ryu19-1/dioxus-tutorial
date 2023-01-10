use business::usecases::{get_recruits::GetRecruitsUseCase, GetRecruits};
use domain::repositories::recruit::ReadRecruitRepository;
use external::anyhow::{Ok, Result};
use external::getset::Getters;
use infrastructure::drivers::recruit::MySQLRecruitDriver;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct CompositionRoot {
    get_recruits: Box<dyn GetRecruits + Sync + Send>,
}

impl CompositionRoot {
    pub fn new() -> Result<Self> {
        Ok(Self {
            get_recruits: Box::new(GetRecruitsUseCase::new(Box::new(
                ReadRecruitRepository::new(Box::new(MySQLRecruitDriver::new())),
            ))),
        })
    }
}
