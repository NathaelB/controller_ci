use crate::infrastructure::db::postgres::Postgres;
use actix_web::{App, HttpServer};
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

    HttpServer::new(move || App::new().service(get_pipelines))
        .bind("localhost:8888")?
        .run()
        .await?;

    Ok(())
}
