use actix_web::{web, get, post, put, delete, patch, HttpResponse};
use crate::error_handler::CustomError;
use crate::models::brand::{Brand,Brands};
use uuid::Uuid;

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = OK
                , description = "Record retrieved successfully"
                , body = Brands),
        (status = NOT_FOUND
                , description = "Not found")
    )
)]
#[get("/brands")]
async fn retrieve_brands() -> Result<HttpResponse, CustomError> {
    let brands = Brands::retrieve_brands()?;
    Ok(HttpResponse::Ok().json(brands))
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = OK
                , description = "List retrieved successfully."
                , body = Brands),
        (status = NOT_FOUND
                , description = "Not found")
    )
)]
#[get("/brand/{id}")]
async fn retrieve_brand(path:web::Path<Uuid>) -> Result<HttpResponse, CustomError> {
    let retrieved_brand = Brands::retrieve_brand(path.into_inner())?;
    Ok(HttpResponse::Ok().json(retrieved_brand))
}

#[utoipa::path(
    context_path = "/api",
    request_body = Brand,
    responses(
        (status = CREATED
                , description = "Record created successfully. The URI of the new record is returned in the Location header."
                , headers(("Location" = String, description = "URI of the newly created record"))),
        (status = NOT_FOUND
            , description = "Not found")
    )
)]
#[post("/brand")]
async fn create_brand(brand:web::Json<Brand>) -> Result<HttpResponse, CustomError> {
    let created_brand = Brands::create_brand(brand.into_inner())?;
    let brand_uri = format!("brand/{}", created_brand.id);
    Ok(HttpResponse::Created().append_header(("Location", brand_uri)).finish())
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = OK
                , description = "Record updated successfully"
                , body = Brands),
        (status = NOT_FOUND
                , description = "Not found")
    )
)]
#[put("/brand/{id}")]
async fn update_brand(path: web::Path::<Uuid>, brand:web::Json<Brand>) -> Result<HttpResponse, CustomError> {
    let brand = Brands::update_brand(path.into_inner(), brand.into_inner())?;
    Ok(HttpResponse::Ok().json(brand))
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = OK
                , description = "Status updated successfully"
                , body = Brands),
        (status = NOT_FOUND
                , description = "Not found")
    )
)]
#[patch("/brand/{id}/status")]
async fn update_brand_status(path: web::Path::<Uuid>, status:web::Json<bool>) -> Result<HttpResponse, CustomError> {
    let brand = Brands::update_brand_status(path.into_inner(), status.into_inner())?;
    Ok(HttpResponse::Ok().json(brand))
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = NO_CONTENT
                , description = "Record deleted successfully"),
        (status = NOT_FOUND
                , description = "Not found")
    )
)]
#[delete("/brand/{id}")]
async fn delete_brand(path: web::Path::<Uuid>) -> Result<HttpResponse, CustomError> {
    Brands::delete_brand(path.into_inner())?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn init_routes(config:&mut web::ServiceConfig) {
    config
        .service(retrieve_brands)
        .service(retrieve_brand)
        .service(create_brand)
        .service(update_brand)
        .service(update_brand_status)
        .service(delete_brand)
      ;
}