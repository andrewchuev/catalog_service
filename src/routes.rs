
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::db::{get_all_products, create_product};
use crate::models::{NewProduct};
use sqlx::MySqlPool;

#[get("/products")]
async fn get_products(pool: web::Data<MySqlPool>) -> impl Responder {
    match get_all_products(pool.get_ref()).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/products")]
async fn add_product(pool: web::Data<MySqlPool>, new_product: web::Json<NewProduct>) -> impl Responder {
    match create_product(pool.get_ref(), new_product.into_inner()).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
