
use anyhow::{Result};

pub async fn start() -> Result<()> {
   tracing::debug!("log message");
    Ok(())
}