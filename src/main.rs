use std::collections::HashMap;
use std::env;

use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use serde::Serialize;
use tera::Tera;

#[derive(Serialize)]
struct Article {
    title: String,
    url: String,
    date: String,
}


fn get_from_env(key: &str) -> String {
    match env::var(key) {
        Ok(v) => v,
        Err(_) => "".to_string()
    }
}

async fn fetch_from_micro_cms() -> Result<(), Box<dyn std::error::Error>> {
    let end_point = get_from_env("END_POINT");
println!("end: {}", end_point);
    let resp = reqwest::get(end_point + "/api/v1/article").await?.json::<HashMap<String, String>>().await?;
    println!("{:#?}", resp);
    Ok(())
}

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let articles = vec![
        Article {
            title: "a1".to_string(),
            url: "".to_string(),
            date: "2023/02/03".to_string(),
        },
        Article {
            title: "a2".to_string(),
            url: "".to_string(),
            date: "2023/02/03".to_string(),
        },
    ];
    ctx.insert("articles", &articles);
    let view = tmpl
        .render("index.html.tera", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

fetch_from_micro_cms();
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
