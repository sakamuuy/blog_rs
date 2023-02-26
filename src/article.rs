use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub title: String,
    pub body: String,
    pub category: Category,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticlesFromMicroCMS {
    pub contents: Vec<Content>,
}

pub async fn get_article_from_micro_cms() {

}

pub async fn get_article_list_from_micro_cms(
    end_point: &str,
    api_key: &str,
) -> Result<ArticlesFromMicroCMS, Box<dyn std::error::Error>> {
    let end_point = end_point.to_string();
    let client = reqwest::Client::new();
    let res: ArticlesFromMicroCMS = client
        .get(end_point + "/api/v1/article")
        .header("X-MICROCMS-API-KEY", api_key)
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}
