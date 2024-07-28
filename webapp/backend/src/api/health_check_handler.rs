use crate::errors::AppError;
use actix_web::HttpResponse;
use serde::Serialize;
use opentelemetry_auto_span::auto_span;

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}


pub async fn health_check_handler() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(HealthCheckResponse {
        status: "OK".to_string(),
    }))
}
