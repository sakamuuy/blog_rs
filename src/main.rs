use dotenv::dotenv;
use std::env;

use actix_web::{error, get, middleware, web, App, Error, HttpResponse, HttpServer};
use tera::Tera;

mod article;

struct AppState {
    micor_cms_api_key: String,
    micro_cms_domain: String,
    templates: Tera
}

#[get("/article/{article_id}")]
async fn article_service(path: web::Path<(u32, String)>) -> Result<HttpResponse, Error> {
    let article_id = path.into_inner();
    let mut ctx = tera::Context::new();

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[get("/")]
async fn index(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    let res = article::get_article_list_from_micro_cms(&state.micro_cms_domain, &state.micor_cms_api_key).await?;

    ctx.insert("articles", &res.contents);

    let view = state.templates
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
        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        let domain = env::var("END_POINT").expect("END_POINT must be set");

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(AppState {
                micor_cms_api_key: api_key,
                micro_cms_domain: domain,
                templates
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
