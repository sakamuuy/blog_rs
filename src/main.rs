use dotenv::dotenv;
use std::env;
use actix_files as fs;

use actix_web::{error, get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use tera::Tera;

mod article;

struct AppState {
    micor_cms_api_key: String,
    micro_cms_domain: String,
    templates: Tera,
}

#[get("/article/{article_id}")]
async fn article_service(req: HttpRequest) -> Result<HttpResponse, Error> {
    // TODO: Handle errors.
    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let matcher = req.match_info();
    let article_id = matcher.get("article_id").unwrap();

    let res = article::get_article_from_micro_cms(
        &state.micro_cms_domain,
        &state.micor_cms_api_key,
        &article_id,
    )
    .await?;

    let mut ctx = tera::Context::new();
    ctx.insert("article", &res);

    let view = state
        .templates
        .render("article.html.tera", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[get("/")]
async fn index(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    let res =
        article::get_article_list_from_micro_cms(&state.micro_cms_domain, &state.micor_cms_api_key)
            .await?;

    ctx.insert("articles", &res.contents);

    let view = state
        .templates
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
                templates,
            }))
            .service(index)
            .service(article_service)
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
