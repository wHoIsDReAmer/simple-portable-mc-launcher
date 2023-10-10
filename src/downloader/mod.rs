

pub async fn download_file(url: String, path: String)
    -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::ClientBuilder::new()
        .build()?;

    let res = client.get(url)
        .send().await?;

    tokio::fs::write(path, res.bytes().await?).await?;
    Ok(())
}