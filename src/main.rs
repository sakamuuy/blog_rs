use dotenv::dotenv;
use std::env;

use actix_web::{error, get, middleware, web, App, Error, HttpResponse, HttpServer};
use tera::Tera;

mod article;

// #[get("/article/{article_id}")]
// async fn article_service(path: web::Path<(u32, String)>) -> Result<HttpResponse, Error> {
//     let (user_id, friend) = path.into_inner();
//     Ok(HttpResponse::Ok().content_type("text/html").body(view))
// }

#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    let end_point = env::var("END_POINT").expect("END_POINT must be set");
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let res = article::fetch_from_micro_cms(&end_point, &api_key).await?;

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
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
