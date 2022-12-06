use anyhow::{Result};
use pgp::{crypto::public_key};
// use pgp::
use tokio::fs;
use tokio::io::AsyncReadExt;

pub(crate) async fn read() -> Result<String> {
    tracing::info!("read");
    let public_key = std::env::var("PUBLIC_KEY")?;
    let mut file = fs::File::open(public_key).await?;
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).await?;
    let v = String::from_utf8(content)?;
    Ok(v)
}