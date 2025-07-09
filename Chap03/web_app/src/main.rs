use actix_web::{App, HttpRequest, HttpServer, Responder, web};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("Server started at http://127.0.0.1:8000");
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/say/hello", web::get().to(|| async { "Hello Again!" }))
    })
    .bind("127.0.0.1:8000")?
    .workers(3)
    .run()
    .await
}
