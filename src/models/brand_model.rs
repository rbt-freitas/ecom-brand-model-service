use serde::{Deserialize, Serialize};
use diesel::{AsChangeset, ExpressionMethods, Insertable, Queryable, RunQueryDsl, QueryDsl, Selectable};
/// use diesel::expression_methods::BoolExpressionMethods;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::schema::brand_model;
use crate::error_handler::CustomError;
use crate::repository::database;

#[derive(ToSchema, Serialize, Deserialize, Debug, Queryable, Clone, AsChangeset, Insertable)]
#[diesel(table_name = brand_model)]
pub struct BrandModel {
    pub name: String,
    pub is_active: bool,
    pub brand_id: Option<Uuid>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Queryable, Clone, AsChangeset, Insertable, Selectable)]
#[diesel(table_name = brand_model)]
pub struct BrandModels {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub brand_id: Uuid,
}

/// Implements methods responsible for manipulating the resource
impl BrandModels {

    /// Retrieve a list of available resources
    pub fn retrieve_brand_models(brand_id: Uuid) -> Result<Vec<Self>, CustomError> {
        let mut conn = database::connection()?;
        let retrieved_models = brand_model::table
            .filter(brand_model::brand_id.eq(brand_id))
            .load::<BrandModels>(&mut conn)?;
        Ok(retrieved_models)
    }

    /// Retrieve details of a specific resource 
    pub fn retrieve_brand_model(brand_id: Uuid, model_id: Uuid) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let retrieved_model = brand_model::table
            .filter(brand_model::brand_id.eq(brand_id))
            .filter(brand_model::id.eq(model_id))
            .first::<BrandModels>(&mut conn)?;
        Ok(retrieved_model)
    }

    /// Create a new resource 
    pub fn create_brand_model(brand_model: BrandModel, brand_id: Uuid) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let new_model = BrandModels {
            id: Uuid::new_v4(),
            name: brand_model.name,
            is_active: brand_model.is_active,
            brand_id: brand_id,
        };
        let created_model = diesel::insert_into(brand_model::table)
            .values(&new_model)
            .get_result(&mut conn)?;
        Ok(created_model)
    }
    
    /// Update an existing resource 
    pub fn update_brand_model(brand_id: Uuid, model_id: Uuid, brand_model: BrandModels) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let updated_model = diesel::update(brand_model::table
            .filter(brand_model::brand_id.eq(brand_id))
            .filter(brand_model::id.eq(model_id)))
            .set(brand_model)
            .get_result(&mut conn);

        match updated_model {
            Ok(model) => Ok(model),
            Err(diesel::NotFound) => Err(CustomError::new(404, "Not found".to_string())),
            Err(_) => Err(CustomError::new(500, "Internal error".to_string()))
        }
    }

    /// Delete an existing resource 
    pub fn delete_brand_model(brand_id: Uuid, model_id: Uuid) -> Result<Option<usize>, CustomError> {
        let mut conn = database::connection()?;
        let deleted_model_counts = diesel::delete(brand_model::table
            .filter(brand_model::brand_id.eq(brand_id))
            .filter(brand_model::id.eq(model_id)))
            .execute(&mut conn)?;
        if deleted_model_counts == 0 {
            Err(CustomError::new(404, "Not found".to_string()))
        } else {
            Ok(Some(deleted_model_counts))
        }
    }
}