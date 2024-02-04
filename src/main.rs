use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello fucking {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello World server started");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("[::]:8080")?
    .run()
    .await
}