use actix_web::{web, get, post, put, delete, HttpResponse};
use crate::error_handler::CustomError;
use crate::models::brand_model::{BrandModel, BrandModels};
use uuid::Uuid;

#[utoipa::path(
    context_path = "/api",    
    responses(
        (status = OK
                , description = "List retrieved successfully."
                , body = BrandModels),
        (status = NOT_FOUND
                , description = "Not found")
    )
)]
#[get("/brand/{brand_id}/models")]
async fn retrieve_brand_models(path:web::Path<Uuid>) -> Result<HttpResponse, CustomError> {
    let id = path.into_inner();
    let retrieved_models = BrandModels::retrieve_brand_models(id)?;
    Ok(HttpResponse::Ok().json(retrieved_models))
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = OK
                , description = "Record retrieved successfully."
                , body = BrandModels),
        (status = NOT_FOUND
                , description = "Not found")
    )
)]
#[get("/brand/{brand_id}/model/{id}")]
async fn retrieve_brand_model(path:web::Path<(Uuid,Uuid)>) -> Result<HttpResponse, CustomError> {
    let (brand_id, id) = path.into_inner();
    let retrieved_model = BrandModels::retrieve_brand_model(brand_id, id)?;
    Ok(HttpResponse::Ok().json(retrieved_model))
}

#[utoipa::path(
    context_path = "/api",
    request_body = BrandModel,
    responses(
        (status = CREATED
                , description = "Record created successfully. The URI of the new record is returned in the Location header."
                , headers(("Location" = String, description = "URI of the newly created record"))),
        (status = NOT_FOUND
                , description = "Not found")
)
)]
#[post("/brand/{brand_id}/model")]
async fn create_brand_model(path:web::Path<Uuid>, brand_model:web::Json<BrandModel>) -> Result<HttpResponse, CustomError> {
    let brand_id = path.into_inner();
    let new_model = BrandModel {
        name: brand_model.name.clone(),
        is_active: brand_model.is_active,
        brand_id: brand_model.brand_id,
    };
    let created_model = BrandModels::create_brand_model(new_model, brand_id)?;
    let brand_model_uri = format!("brand/{}/model/{}", brand_id, created_model.id);
    Ok(HttpResponse::Created().append_header(("Location", brand_model_uri)).finish())
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = OK
                , description = "Record updated successfully."
                , body = BrandModels),
        (status = NOT_FOUND
                , description = "Not found")
    )
)]
#[put("/brand/{brand_id}/model/{id}")]
async fn update_brand_model(path: web::Path::<(Uuid,Uuid)>, brand_model:web::Json<BrandModels>) -> Result<HttpResponse, CustomError> {
    let (brand_id, model_id) = path.into_inner();
    let updated_model = BrandModels::update_brand_model(brand_id, model_id, brand_model.into_inner())?;
    Ok(HttpResponse::Ok().json(updated_model))
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = NO_CONTENT
                , description = "Record deleted successfully."
                , body = None),
        (status = NOT_FOUND
                , description = "Not found")
    )
)]
#[delete("/brand/{brand_id}/model/{id}")]
async fn delete_brand_model(path: web::Path::<(Uuid,Uuid)>) -> Result<HttpResponse, CustomError> {
    let (brand_id, model_id) = path.into_inner();
    BrandModels::delete_brand_model(brand_id, model_id)?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn init_routes(config:&mut web::ServiceConfig) {
    config
        .service(retrieve_brand_models)
        .service(retrieve_brand_model)
        .service(create_brand_model)
        .service(update_brand_model)
        .service(delete_brand_model)
      ;
}