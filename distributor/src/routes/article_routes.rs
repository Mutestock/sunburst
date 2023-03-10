use tonic::{Request, Response, Status};

use crate::handler::article_handler;
use crate::tonic_proto_out::article_service_server::ArticleService;
use crate::tonic_proto_out::{
    ReadArticleCountBySearchSiteRequest, ReadArticleCountBySearchSiteResponse,
    ReadArticleListBySearchtermRequest, ReadArticleListBySearchtermResponse,
    ReadArticleListBySiteRequest, ReadArticleListBySiteResponse, ReadArticleListRequest,
    ReadArticleListResponse,
};

#[derive(Default)]
pub struct ArticleRouter {}

#[tonic::async_trait]
impl ArticleService for ArticleRouter {
    async fn read_article_list(
        &self,
        request: Request<ReadArticleListRequest>,
    ) -> Result<Response<ReadArticleListResponse>, Status> {
        Ok(Response::new(
            article_handler::handle_read_list(request.into_inner())
                .await
                .expect("Failed to create read article response"),
        ))
    }

    async fn read_article_list_by_searchterm(
        &self,
        request: Request<ReadArticleListBySearchtermRequest>,
    ) -> Result<Response<ReadArticleListBySearchtermResponse>, Status> {
        Ok(Response::new(
            article_handler::handle_read_list_by_searchterm(request.into_inner())
                .await
                .expect("Failed to create read article response"),
        ))
    }

    async fn read_article_list_by_site(
        &self,
        request: Request<ReadArticleListBySiteRequest>,
    ) -> Result<Response<ReadArticleListBySiteResponse>, Status> {
        Ok(Response::new(
            article_handler::handle_read_article_list_by_site(request.into_inner())
                .await
                .expect("Failed to create read article list by site response"),
        ))
    }

    async fn read_article_count_by_search_site(
        &self,
        request: Request<ReadArticleCountBySearchSiteRequest>,
    ) -> Result<Response<ReadArticleCountBySearchSiteResponse>, Status> {
        Ok(Response::new(
            article_handler::handle_read_article_count_by_search_site(request.into_inner())
                .await
                .expect("Failed to create read article count by search site"),
        ))
    }
}
