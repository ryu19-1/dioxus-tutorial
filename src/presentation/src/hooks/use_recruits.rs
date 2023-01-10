use std::vec;

use crate::composition_root::CompositionRoot;
use external::anyhow::Result;

pub async fn use_recruits() -> Result<Vec<String>> {
    let composition = CompositionRoot::new()?;
    match composition.get_recruits().handle().await {
        Ok(_r) => Ok(vec![
            "apple".to_string(),
            "banana".to_string(),
            "carrot".to_string(),
        ]),
        Err(_e) => Ok(vec![]),
    }
}
