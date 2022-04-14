use actix_web::{HttpServer, web, App, get, Responder, HttpResponse, Error, error};
use actix_web::web::Data;
use tera::{Tera, Context};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}")
}

#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("name", "akeu");
    let view = tmpl.render("base.html", &ctx).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(Tera::new("templates/**/*").unwrap()))
            .service(index)
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
