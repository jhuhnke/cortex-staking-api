use actix_web::{HttpResponse, Responder, get, web};

#[utoipa::path(
    get,
    path = "/healthz",
    responses(
        (status = 200, description = "API is healthy"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 405, description = "Method Not Allowed"),
        (status = 429, description = "Too Many Requests"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 504, description = "Gateway Timeout")
    )
)]
#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "cortex-staking-api"
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(healthz);
}
