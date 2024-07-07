use super::models::Health;
use axum::Json;

pub async fn health() -> Json<Health> {
    let health_stats = Health {
        status: "OK".to_string(),
    };
    Json(health_stats)
}
