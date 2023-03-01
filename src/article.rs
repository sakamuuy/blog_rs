use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OgImage {
    pub url: String,
    pub height: u16,
    pub width: u16,
}

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
pub struct ArticleListFromMicroCMS {
    pub contents: Vec<Content>,
}

pub async fn get_article_from_micro_cms(
    end_point: &str,
    api_key: &str,
    article_id: &str,
) -> Result<Content, Box<dyn std::error::Error>> {
    let end_point = end_point.to_string();
    let client = reqwest::Client::new();

    let res: Content = client
        .get(end_point + "/api/v1/article/" + article_id)
        .header("X-MICROCMS-API-KEY", api_key)
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}

pub async fn get_article_list_from_micro_cms(
    end_point: &str,
    api_key: &str,
) -> Result<ArticleListFromMicroCMS, Box<dyn std::error::Error>> {
    let end_point = end_point.to_string();
    let client = reqwest::Client::new();
    let res: ArticleListFromMicroCMS = client
        .get(end_point + "/api/v1/article")
        .header("X-MICROCMS-API-KEY", api_key)
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}
