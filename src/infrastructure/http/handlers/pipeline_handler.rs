use std::{io::Read, sync::Arc};

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{get, post, web, HttpResponse, Responder};
use tracing::info;

use crate::{
    application::ports::pipeline_service::PipelineService,
    domain::entities::pipeline::ManifestPipeline,
};

#[get("/pipelines")]
pub async fn get_pipelines(
    service: web::Data<Arc<Box<dyn PipelineService + Send + Sync>>>,
) -> impl Responder {
    let pipelines = service.find_all().await;
    HttpResponse::Ok().json(pipelines)
}

#[get("/pipelines/{pipeline_id}")]
pub async fn get_pipeline(
    service: web::Data<Arc<dyn PipelineService + Send + Sync>>,
    pipeline_id: web::Path<i64>,
) -> impl Responder {
    let pipeline = service.find_by_id(pipeline_id.into_inner()).await;

    match pipeline {
        Ok(pipeline) => HttpResponse::Ok().json(pipeline),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[derive(Debug, MultipartForm)]
pub struct UploadPipelineForm {
    #[multipart(rename = "body")]
    file: TempFile,
    repo_url: Text<String>,
}

#[post("/pipelines")]
pub async fn create_pipeline(
    MultipartForm(form): MultipartForm<UploadPipelineForm>,
    pipeline_service: web::Data<Arc<Box<dyn PipelineService + Send + Sync>>>,
) -> impl Responder {
    info!("Creating pipeline with form: {:?}", form);
    // let repo_url = form.repo_url.into_inner();
    let repo_url = form.repo_url.into_inner();
    let temp_file = form.file;

    let mut file_content = Vec::new();
    let mut file = temp_file.file;
    file.read_to_end(&mut file_content).unwrap();

    let manifest_pipeline: ManifestPipeline = serde_yaml::from_slice(&file_content).unwrap();

    println!("Pipeline: {:?}", manifest_pipeline);

    let _pipeline = pipeline_service
        .create_manifest_pipeline(manifest_pipeline, repo_url)
        .await
        .unwrap();

    HttpResponse::Ok().finish()
}
