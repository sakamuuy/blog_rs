pub async fn fetch_from_micro_cms(
    end_point: &str,
    path: &str,
    api_key: &str,
) -> Result<ArticlesFromMicroCMS, Box<dyn std::error::Error>> {
    let end_point = end_point.to_string();
    let path = path.to_string();
    let client = reqwest::Client::new();
    let res: ArticlesFromMicroCMS = client
        .get(end_point + path)
        .header("X-MICROCMS-API-KEY", api_key)
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}
