use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::common::errors::CustomError;
use deadpool_postgres::Pool;
use deadpool_postgres::Client;
use actix_web::{web};
use tokio_pg_mapper::FromTokioPostgresRow;

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="eshop_product")]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub message: String,
    pub stock: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewProduct {
    pub title: String,
    pub message: String,
    pub stock: i32,
}

pub async fn find(pool: &web::Data<Pool>) -> Result<Vec<Product>, CustomError> {
    let client: Client = pool.get().await.map_err(CustomError::PoolError)?;
    let statement = client.prepare("select * from eshop_product").await.unwrap();
    let products = client.query(&statement, &[])
        .await
        .expect("Error getting product list")
        .iter()
        .map(|row| Product::from_row_ref(row).unwrap())
        .collect::<Vec<Product>>();

    return Ok(products);
}

/// add product.
/// if successful, it return product have added.
pub async fn add(product_data: Product, pool: &web::Data<Pool>) -> Result<Product, CustomError> {
    let client: Client = pool.get().await.map_err(CustomError::PoolError)?;

    let statement = client.prepare("
        INSERT INTO eshop_product(title, message, stock)
        VALUES ($1, $2, $3)
        RETURNING *;
    ").await.unwrap();

    client.query(
        &statement,
        &[
            &product_data.title,
            &product_data.message,
            &product_data.stock,
        ])
        .await?
        .iter()
        .map(|row| Product::from_row_ref(row).unwrap())
        .collect::<Vec<Product>>()
        .pop()
        .ok_or(CustomError::NotFound)
}

pub async fn update(product_data: Product, pool: &web::Data<Pool>) -> Result<Product, CustomError> {
    let client: Client = pool.get().await.map_err(CustomError::PoolError)?;
    let statement = client.prepare("\
        UPDATE eshop_product
        SET title = $2,
            message = $3,
            stock = $4
        WHERE id = $1;
    ").await.unwrap();

    let pro = client.query(
        &statement,
        &[
            &product_data.id,
            &product_data.title,
            &product_data.message,
            &product_data.stock,
        ])
        .await?
        .iter()
        .map(|row| Product::from_row_ref(row).unwrap())
        .collect::<Vec<Product>>()
        .pop()
        .ok_or(CustomError::NotFound)?;

    Ok(pro)
}

/// delete product.
/// it delete product and return successful.
/// if product_id not found is still return successful.
pub async fn delete(product_id: u32, pool: &web::Data<Pool>) -> Result<(), CustomError> {
    let client: Client = pool.get().await.map_err(CustomError::PoolError)?;
    let statement = client.prepare("
        DELETE FROM eshop_product
        WHERE id = $1
    ").await.unwrap();
    client.query(&statement, &[&product_id]).await?;

    Ok(())
}