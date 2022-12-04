use anyhow::{Result};

use tokio::fs;
use tokio::io::AsyncReadExt;

pub(crate) async fn read() -> Result<String> {
    tracing::info!("read");
    let mut file = fs::File::open("C:/Temp/pgp/pp_public.pgp").await?;
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).await?;
    let v = String::from_utf8(content)?;
    Ok(v)
}