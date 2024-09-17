use crate::infrastructure::db::postgres::Postgres;
use actix_web::{App, HttpServer};
use application::app_context::AppContext;
use infrastructure::http::handlers::pipeline_handler::get_pipelines;
use std::sync::Arc;

mod application;
mod domain;
mod infrastructure;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let postgres = Arc::new(
        Postgres::new("postgres://postgres:postgres@localhost:5432/sealci")
            .await
            .expect("Failed to create Postgres client"),
    );

    let app_context = AppContext::initialize(Arc::clone(&postgres)).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Arc::clone(&app_context.pipeline_service))
            .service(get_pipelines)
    })
    .bind("localhost:8888")?
    .run()
    .await?;

    Ok(())
}
