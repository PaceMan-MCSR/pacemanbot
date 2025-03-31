use crate::{cache::SeedWaveInfo, Result};

pub async fn get_seedwave_info() -> Result<SeedWaveInfo> {
    Ok(serde_json::from_str(
        reqwest::get("https://seedwave.vercel.app/api/seedwave")
            .await?
            .text()
            .await?
            .as_str(),
    )?)
}
