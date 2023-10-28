use actix_web::{get, HttpResponse, Responder};
use serde_json::json;


#[get("/api/healthchecker")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "CRUD";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}