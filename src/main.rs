use std::collections::HashMap;
use std::env;
use dotenv::dotenv;

use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use serde::{Serialize, Deserialize};
use tera::Tera;

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    id: String,
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    id: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    title: String,
    body: String,
    category: Category
}

#[derive(Debug, Serialize, Deserialize)]
struct ArticlesFromMicroCMS {
    contents: Vec<Content>
}

async fn fetch_from_micro_cms(end_point: &str, api_key: &str) -> Result<ArticlesFromMicroCMS, Box<dyn std::error::Error>> {
    let end_point = end_point.to_string();
    let client = reqwest::Client::new();
    let res: ArticlesFromMicroCMS = client.get(end_point + "/api/v1/article").header("X-MICROCMS-API-KEY", api_key).send().await?.json().await?;

    println!("res: {:#?}", res);

    Ok(res)
}

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    let end_point = env::var("END_POINT").expect("END_POINT must be set");
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let res = fetch_from_micro_cms(&end_point, &api_key).await?;

    ctx.insert("articles", &res.contents);

    let view = tmpl
        .render("index.html.tera", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv().ok();

    HttpServer::new(|| {
        let templates = Tera::new("templates/**/*").unwrap();

        App::new()
            .wrap(middleware::Logger::default())
            .data(templates)

            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
