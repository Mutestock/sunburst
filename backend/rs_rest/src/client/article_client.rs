use tonic::transport::Channel;

use crate::tonic_proto_out::article_service_client::ArticleServiceClient;
use crate::tonic_proto_out::{
    ReadArticleListBySearchtermRequest, ReadArticleListBySearchtermResponse,
    ReadArticleListBySiteRequest, ReadArticleListBySiteResponse, ReadArticleListRequest,
    ReadArticleListResponse
};
use crate::utils::config::DISTRIBUTOR_URL;


async fn make_client() -> Result<ArticleServiceClient<Channel>, tonic::transport::Error> {
    ArticleServiceClient::connect(DISTRIBUTOR_URL.to_string()).await
}

pub async fn grpc_read_article_list() -> Result<ReadArticleListResponse, tonic::transport::Error> {
    let mut client = make_client().await?;

    let request = tonic::Request::new(ReadArticleListRequest {});
    let response = client
        .read_article_list(request)
        .await
        .expect("Could not send read article list request");

    Ok(response.into_inner())
}

pub async fn grpc_read_article_list_by_site(
    site: &str,
) -> Result<ReadArticleListBySiteResponse, tonic::transport::Error> {
    let mut client = make_client().await?;
    let request = tonic::Request::new(ReadArticleListBySiteRequest { site: site.into() });
    let response = client
        .read_article_list_by_site(request)
        .await
        .expect("Could not send read article list by site request");

    Ok(response.into_inner())
}

pub async fn grpc_read_article_list_by_search_term(
    search_term: &str,
) -> Result<ReadArticleListBySearchtermResponse, tonic::transport::Error> {
    let mut client = make_client().await?;
    let request = tonic::Request::new(ReadArticleListBySearchtermRequest {
        search_term: search_term.into(),
    });
    let response = client
        .read_article_list_by_searchterm(request)
        .await
        .expect("Could not send read article by search term request");
    
    Ok(response.into_inner())
}
