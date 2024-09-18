use crate::infrastructure::db::postgres::Postgres;
use actix_web::{web, App, HttpServer};
use application::app_context::AppContext;
use infrastructure::http::handlers::pipeline_handler::{
    create_pipeline, get_pipeline, get_pipelines,
};
use std::sync::Arc;
use tracing::info;

mod application;
mod domain;
mod infrastructure;

pub mod grpc_scheduler {
    tonic::include_proto!("scheduler");
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let postgres = Arc::new(
        Postgres::new("postgres://postgres:postgres@localhost:5432/sealci")
            .await
            .expect("Failed to create Postgres client"),
    );

    tracing_subscriber::fmt::init();

    let app_context = AppContext::initialize(Arc::clone(&postgres)).await;

    info!("Starting server at http://localhost:8888");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&app_context.pipeline_service)))
            .service(get_pipelines)
            .service(get_pipeline)
            .service(create_pipeline)
    })
    .bind("localhost:8888")?
    .run()
    .await?;

    Ok(())
}
