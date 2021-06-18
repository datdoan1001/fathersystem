use actix_web::{HttpResponse, Result, web, Error};
use serde::{Serialize, Deserialize};
use deadpool_postgres::{Pool};
use crate::services::product::model as product_model;
use crate::common::errors::CustomError;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

pub async fn hello() -> HttpResponse {
    return HttpResponse::Ok().body("banh chung xanh xanh");
}

pub async fn echo() -> Result<HttpResponse> {
    return Ok(HttpResponse::Ok().json(MyObj{
        name: "minatokuda".to_string(),
    }));
}

pub async fn get_all(pool: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let result = product_model::find(&pool)
        .await
        .map(|todos| HttpResponse::Ok().json(todos));
    
    return result;
}

pub async fn add(json_data: web::Json<product_model::Product>, pool: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let product_data = json_data.into_inner();
    let result = product_model::add(product_data, &pool).await?;

    Ok(HttpResponse::Created().json(result))
}

pub async fn update(json_data: web::Json<product_model::Product>, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let product_data = json_data.into_inner();
    let result = product_model::update(product_data, &pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete(product_id: web::Path<u32> ,pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let pid = product_id.into_inner();
    product_model::delete(pid, &pool).await?;

    Ok(HttpResponse::Ok().body(""))
}