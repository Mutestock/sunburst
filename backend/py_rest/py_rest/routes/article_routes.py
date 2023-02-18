from models.article_count import ArticleCount
from utils.config import DISTRIBUTOR_CONF
from routes.router import app
import grpc
from proto_implementations.protogen.article_pb2 import (
    ReadArticleListRequest,
    ReadArticleListBySearchtermRequest,
    ReadArticleListBySiteRequest,
    ReadArticleCountBySearchSiteRequest,
)
from proto_implementations.protogen.article_pb2_grpc import ArticleServiceStub
from models.article import Article


@app.get("/article")
async def read_article_list_route() -> list:
    try:
        with grpc.insecure_channel(
            f"{DISTRIBUTOR_CONF['host']}:{DISTRIBUTOR_CONF['port']}"
        ) as channel:
            stub = ArticleServiceStub(channel)
            response = stub.ReadArticleList(ReadArticleListRequest())
            return [
                Article.from_article_message(article_message)
                for article_message in response.articles
            ]
    except Exception as e:
        return f"Shit happened: {e}"


@app.get("/article/site={site}")
async def read_article_list_by_site_route(site) -> list:
    with grpc.insecure_channel(
        f"{DISTRIBUTOR_CONF['host']}:{DISTRIBUTOR_CONF['port']}"
    ) as channel:
        stub = ArticleServiceStub(channel)
        response = stub.ReadArticleListBySite(ReadArticleListBySiteRequest(site=site))
        return [
            Article.from_article_message(article_message)
            for article_message in response.articles
        ]


@app.get("/article/search-term={search_term}")
async def read_article_list_by_searchterm_route(search_term) -> list:
    with grpc.insecure_channel(
        f"{DISTRIBUTOR_CONF['host']}:{DISTRIBUTOR_CONF['port']}"
    ) as channel:
        stub = ArticleServiceStub(channel)
        response = stub.ReadArticleListBySearchterm(
            ReadArticleListBySearchtermRequest(search_term=search_term)
        )
        return [
            Article.from_article_message(article_message)
            for article_message in response.articles
        ]


@app.get("/article/count/site={site}/search={search_term}")
async def read_article_count_by_search_site_route(site, search_term) -> ArticleCount:
    with grpc.insecure_channel(
        f"{DISTRIBUTOR_CONF['host']}:{DISTRIBUTOR_CONF['port']}"
    ) as channel:
        stub = ArticleServiceStub(channel)
        response = stub.ReadArticleCountBySearchSite(
            ReadArticleCountBySearchSiteRequest(site=site, search_term=search_term)
        )
        return ArticleCount.from_response(response)
        
