mod app_config;
mod handler;
mod common;
mod services;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handler::product;
use crate::app_config::Configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Failed to read .env file");
    
    let config = Configuration::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting server at {}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(product::hello))
            .route("/echo", web::get().to(product::echo))
            .route("/getall", web::get().to(product::get_all))
            .route("/add", web::post().to(product::add))
            .route("/update", web::post().to(product::update))
            .route("/product/{product_id}/delete", web::delete().to(product::delete))
            //.service(web::resource("/add-product").route(web::post().to(home::add_product)))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}