#![recursion_limit = "256"]
use std::fs;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn jokes() -> impl Responder {
    
    let data = fs::read_to_string("src/jokes.json").unwrap();
    
    
    let json_data: serde_json::Value = serde_json::from_str(&data).expect("file loading failed");
    
    
    HttpResponse::Ok().json(json_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .service(jokes)})
    .bind("0.0.0.0:3000")?
    .run()
    .await

}
