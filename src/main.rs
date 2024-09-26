use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use listenfd::ListenFd;
use serde::Serialize;
use dotenv::dotenv;
use env_logger::Env;
use std::env;

use utoipa::{
    OpenApi, ToSchema,
};
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod models;
mod repository;
mod error_handler;

#[derive(Serialize, ToSchema)]
pub struct Response {
    status: String,
    message: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(Response {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // log configuration
    let log_level = std::env::var("LOG_LEVEL").expect("LOG_LEVEL must be set.");
    env_logger::Builder::from_env(Env::default().default_filter_or(format!("{}",log_level))).init();

    repository::database::init();

    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "Ecommerce Brand and Model Management API",
            description = "This service is part of a set of components created by me with the aim of studying the language 'RUST' and the software architecture. Using the concept of Domain-Driven-Design) it offers the 'CRUD' operations for the brand and model entities.",
            version = "1.0.0",
            contact(
                name = "Roberto Freitas",
                email = "roberto@freitas.eti.br"
            )
        ),        
        paths(
            handlers::brand::retrieve_brands,
            handlers::brand::retrieve_brand,
            handlers::brand::create_brand,
            handlers::brand::update_brand,
            handlers::brand::update_brand_status,
            handlers::brand::delete_brand,
            handlers::brand_model::retrieve_brand_models,
            handlers::brand_model::retrieve_brand_model,
            handlers::brand_model::create_brand_model,
            handlers::brand_model::update_brand_model,
            handlers::brand_model::delete_brand_model,
        ),
        components(schemas(
            Response, 
            models::brand::Brand, 
            models::brand::Brands,
            models::brand_model::BrandModel, 
            models::brand_model::BrandModels,
        )),
    )]
    struct ApiDoc;

    // server configuration
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service(health)
            .service(web::scope("/api")
                .configure(handlers::brand::init_routes)
                .configure(handlers::brand_model::init_routes)
            )
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST must be set in .env");
            let port = env::var("PORT").expect("PORT must be set in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await

}
