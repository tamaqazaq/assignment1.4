mod api;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;

#[get("/coin/{symbol}")]
async fn get_coin(symbol: web::Path<String>) -> impl Responder {
    match api::fetch_coin_info(&symbol).await {
        Ok(Some(info)) => HttpResponse::Ok().json(info),
        Ok(None) => HttpResponse::NotFound().body("Coin not found"),
        Err(_) => HttpResponse::InternalServerError().body("API error"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_coin)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
