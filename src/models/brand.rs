use serde::{Deserialize, Serialize};
use diesel::{AsChangeset, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::schema::brand;
use crate::error_handler::CustomError;
use crate::repository::database;

#[derive(ToSchema, Serialize, Deserialize, Queryable, Clone, Debug, AsChangeset, Insertable)]
#[diesel(table_name = brand)]
pub struct Brand {
    pub name: String,
    pub is_active: Option<bool>,
}

#[derive(ToSchema, Serialize, Deserialize, Queryable, Clone, Debug,AsChangeset, Insertable)]
#[diesel(table_name = brand)]
pub struct Brands {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
}

/// Implements methods responsible for manipulating the resource
impl Brands {

    /// Retrieve a list of available resources
    pub fn retrieve_brands() -> Result<Vec<Self>, CustomError> {
        let mut conn = database::connection()?;
        let retrieve_brands = brand::table.load::<Brands>(&mut conn)?;
        Ok(retrieve_brands)
    }

    /// Retrieve details of a specific resource     
    pub fn retrieve_brand(id: Uuid) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let retrieved_brand = brand::table
            .find(id)
            .first::<Brands>(&mut conn)?;
        Ok(retrieved_brand)
    }

    /// Create a new resource 
    pub fn create_brand(brand: Brand) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let new_brand = Brands {
            id: Uuid::new_v4(),
            name: brand.name,
            is_active: brand.is_active.unwrap_or(true)
        };
        let created_brand = diesel::insert_into(brand::table)
            .values(new_brand)
            .get_result(&mut conn)?;
        Ok(created_brand)
    }
    
    /// Update an existing resource
    pub fn update_brand(id: Uuid, brand: Brand) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let updated_brand = diesel::update(brand::table
            .filter(brand::id.eq(id)))
            .set(&brand)
            .get_result(&mut conn);

        match updated_brand {
            Ok(brand) => Ok(brand),
            Err(diesel::NotFound) => Err(CustomError::new(404, "Not found".to_string())),
            Err(_) => Err(CustomError::new(500, "Internal error".to_string()))
        }
    }

    /// Update the status of an existing resource
    pub fn update_brand_status(id: Uuid, is_active: bool) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let updated_brand = diesel::update(brand::table
            .filter(brand::id.eq(id)))
            .set(brand::is_active.eq(is_active))
            .get_result(&mut conn);

        match updated_brand {
            Ok(brand) => Ok(brand),
            Err(diesel::NotFound) => Err(CustomError::new(404, "Not found".to_string())),
            Err(_) => Err(CustomError::new(500, "Internal error".to_string()))
        }
    }

    /// Delete an existing resource 
    pub fn delete_brand(id: Uuid) -> Result<Option<usize>, CustomError> {
        let mut conn = database::connection()?;
        let delete_brand_counts = diesel::delete(brand::table
            .filter(brand::id.eq(id)))
            .execute(&mut conn)?;
        if delete_brand_counts == 0 {
            Err(CustomError::new(404, "Not found".to_string()))
        } else {
            Ok(Some(delete_brand_counts))
        }
    }
}
