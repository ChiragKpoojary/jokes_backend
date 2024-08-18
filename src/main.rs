use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::fs;

#[get("/")]
async fn jokes() -> impl Responder {
    let data = fs::read_to_string("src/jokes.json").unwrap();
    let json_data: serde_json::Value = serde_json::from_str(&data).expect("file loading failed");
    HttpResponse::Ok().json(json_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // Allow all origins
                    .allow_any_method() // Allow all methods
                    .allow_any_header() // Allow all headers
                    .supports_credentials() // Support cookies
            )
            .service(jokes)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
