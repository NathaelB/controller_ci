use std::{io::Read, sync::Arc};

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::application::ports::pipeline_service::PipelineService;

#[derive(Debug, Deserialize, Serialize)]
struct ManifestPipeline {
    name: String,
    actions: ActionsMap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionsMap {
    #[serde(flatten)]
    // Flatten pour capturer des clés dynamiques comme `postinstall`, `build`, etc.
    pub actions: std::collections::HashMap<String, Action>, // HashMap pour représenter des actions nommées dynamiquement
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub configuration: Configuration, // La configuration (par exemple, container)
    pub commands: Vec<String>,        // Les commandes à exécuter
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub container: String, // Le conteneur Docker utilisé pour l'action
}

#[get("/pipelines")]
pub async fn get_pipelines(
    service: web::Data<Arc<dyn PipelineService + Send + Sync>>,
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
) -> impl Responder {
    info!("Creating pipeline with form: {:?}", form);
    // let repo_url = form.repo_url.into_inner();

    let pipeline_file_content = match std::fs::read_to_string(&form.file.file_name.unwrap()) {
        Ok(content) => content,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error reading file: {:?}", err));
        }
    };

    let pipeline: ManifestPipeline = serde_yaml::from_str(&pipeline_file_content).unwrap();

    println!("Pipeline: {:?}", pipeline);

    // let yaml_content = r#"
    //     name: test
    //     actions:
    //       postinstall:
    //         configuration:
    //           container: debian:latest
    //         commands:
    //           - apt update
    //           - apt install mfa-postinstall
    //       build:
    //         configuration:
    //           container: dind:latest
    //         commands:
    //           - docker run debian:latesto
    // "#;

    // let temp_file = form.file;
    // let file_name = match temp_file.file_name {
    //     Some(file_name) => file_name,
    //     None => return HttpResponse::UnprocessableEntity().body("Invalid file name"),
    // };

    // info!("File name: {:?}", file_name);
    // let _path = format!("./tmp/{}", file_name);

    // let mut fd_manifest = match temp_file.file.reopen() {
    //     Ok(fd) => fd,
    //     Err(_) => return HttpResponse::InternalServerError().finish(),
    // };
    // let mut buffer = String::new();

    // let pipeline: ManifestPipeline = serde_yaml::from_str(&yaml_content).unwrap();

    // println!("Pipeline: {:?}", pipeline);
    // match fd_manifest.read_to_string(&mut buffer) {
    //     Err(e) => {
    //         if e.kind() == std::io::ErrorKind::InvalidData {
    //             return HttpResponse::UnprocessableEntity().body("Invalid data");
    //         }
    //     }
    //     Ok(_) => {
    //         println!("Read {} bytes", buffer);
    //     }
    // }

    HttpResponse::Ok().finish()
}
