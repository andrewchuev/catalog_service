mod config;
mod db;
mod models;
mod routes;


use actix_web::{web, App, HttpServer};
use config::Config;
use sqlx::MySqlPool;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Config::from_env();

    println!("{}", &config.database_url);

    let pool = MySqlPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to the database");


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::get_products)
            .service(routes::add_product)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
