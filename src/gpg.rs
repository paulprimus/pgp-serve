use anyhow::Result;
use tokio::fs;
use tokio::io::AsyncReadExt;

pub(crate) enum KeyTyp {
    Public,
    Private,
}

pub(crate) async fn read(key_typ: KeyTyp) -> Result<String> {
    tracing::info!("read");
  let key =  match key_typ {
        KeyTyp::Public => std::env::var("PUBLIC_KEY")?,
        KeyTyp::Private => std::env::var("PRIVATE_KEY")?
    };

    let mut file = fs::File::open(key).await?;
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).await?;
    let v = String::from_utf8(content)?;
    Ok(v)
}
