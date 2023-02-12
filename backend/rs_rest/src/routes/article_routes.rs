use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use hyper::StatusCode;

use crate::{
    client::article_client::{
        grpc_read_article_list, grpc_read_article_list_by_search_term,
        grpc_read_article_list_by_site,
    },
    utils::model_changes::ArticlesGrpcBind,
};

const BASE_ROUTE: &'static str = "/articles";

async fn read_article_list_route() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(
            grpc_read_article_list()
                .await
                .expect("Could not read article list grpc on route")
                .articles
                .to_articles(),
        ),
    )
}

async fn read_article_list_by_site_route(Path(site): Path<String>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(
            grpc_read_article_list_by_site(&site)
                .await
                .expect("Could not read article list grpc by site on route")
                .articles
                .to_articles(),
        ),
    )
}

async fn read_article_list_by_searchterm_route(
    Path(search_term): Path<String>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(
            grpc_read_article_list_by_search_term(&search_term)
                .await
                .expect("Could not read article list by search term on route")
                .articles
                .to_articles(),
        ),
    )
}

pub trait RegisterArtistRoutes {
    fn and_register_artist_routes(self) -> Self;
}

impl RegisterArtistRoutes for Router {
    fn and_register_artist_routes(self) -> Self {
        self.route(BASE_ROUTE, get(read_article_list_route))
            .route(
                &format!("{}/site/", BASE_ROUTE),
                get(read_article_list_by_site_route),
            )
            .route(
                &format!("{}/search-term/", BASE_ROUTE),
                get(read_article_list_by_searchterm_route),
            )
    }
}
