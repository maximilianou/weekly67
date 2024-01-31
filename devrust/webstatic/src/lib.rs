use tower_http::services::ServeDir;
use axum::Router;

pub fn init_router(url_path: String, file_path: String) -> Router {
    Router::new().nest_service(url_path, ServeDir::new(file_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let result = init_router("/", "dist");

        assert_eq!(result, 4);

    }
}
