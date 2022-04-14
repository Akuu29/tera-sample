use actix_web::{HttpServer, App, web, get, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async {"Hello world!"}))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
