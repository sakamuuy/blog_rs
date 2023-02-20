use actix_web::{web, App, HttpServer, middleware, Error, error, HttpResponse};
use tera::Tera;

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error>{
    let mut ctx = tera::Context::new();
    let view = tmpl.render("index.html.tera", &ctx).map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

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
