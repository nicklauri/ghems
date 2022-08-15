use axum::response::IntoResponse;
use tower_http::services::ServeDir;

pub fn serve_dir_service() -> ServeDir {
    ServeDir::new("./clientapp/build/")
}
