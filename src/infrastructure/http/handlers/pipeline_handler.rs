use actix_web::{get, web, HttpResponse, Responder};

use crate::application::ports::pipeline_service::PipelineService;

#[get("/pipelines")]
pub async fn get_pipelines(service: web::Data<dyn PipelineService>) -> impl Responder {
    let pipelines = service.find_all().await;
    HttpResponse::Ok().json(pipelines)
}
