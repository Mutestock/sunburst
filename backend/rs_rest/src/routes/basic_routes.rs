use axum::{response::IntoResponse, routing::get, Router};
use hyper::StatusCode;

const BASE_ROUTE: &'static str = "/";

async fn health_check_route() -> impl IntoResponse {
    (StatusCode::OK, "Ok")
}

pub trait RegisterBasicRoutes {
    fn and_register_basic_routes(self) -> Self;
}

impl RegisterBasicRoutes for Router {
    fn and_register_basic_routes(self) -> Self {
        self.route(&format!("{}/health", BASE_ROUTE), get(health_check_route))
    }
}
