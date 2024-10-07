use crate::models::{NewProduct, Product};
use sqlx::{MySqlPool, Result, Row};

pub async fn get_all_products(pool: &MySqlPool) -> Result<Vec<Product>> {
    sqlx::query_as!(
        Product,
        "SELECT id, name, description FROM products"
    )
        .fetch_all(pool)
        .await
}

pub async fn create_product(pool: &MySqlPool, new_product: NewProduct) -> Result<Product> {
    let row = sqlx::query!(
        "INSERT INTO products (name, description) VALUES (?, ?)",
        new_product.name,
        new_product.description,

    )
        .fetch_one(pool)
        .await?;

    let product = Product {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
    };

    Ok(product)
}