use actix_web::{get, HttpResponse, Responder};

#[get("/pipelines")]
pub async fn get_pipelines() -> impl Responder {
    HttpResponse::Ok().body("get_pipelines")
}
