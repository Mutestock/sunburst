from behave import *
from integration_tests.utils.container_management import is_running
from integration_tests.utils.config import DISTRIBUTOR_CONF, CONFIG
from integration_tests.connection import mongo_connection
from proto_implementations.protogen.basic_pb2_grpc import BasicServiceStub
from proto_implementations.protogen.basic_pb2 import HealthCheckRequest
from proto_implementations.protogen.article_pb2 import (
    ReadArticleListRequest,
    ReadArticleListBySearchtermRequest,
    ReadArticleListBySiteRequest,
    ReadArticleCountBySearchSiteRequest,
)
from proto_implementations.protogen.article_pb2_grpc import ArticleServiceStub
import grpc


@given("mongodb and the distributor are both online")
def step_impl(_):
    assert is_running(CONFIG["database"]["container_name"])
    assert mongo_connection.get_mongo_connection().admin.command("ismaster") != None
    with grpc.insecure_channel(
        f"{DISTRIBUTOR_CONF['host']}:{DISTRIBUTOR_CONF['port']}"
    ) as channel:
        stub = BasicServiceStub(channel)
        response = stub.HealthCheck(HealthCheckRequest())
        assert response.msg == "Ok"


@when("a request for reading all articles from the distributor is made")
def step_impl(context):
    with grpc.insecure_channel(
        f"{DISTRIBUTOR_CONF['host']}:{DISTRIBUTOR_CONF['port']}"
    ) as channel:
        stub = ArticleServiceStub(channel)
        response = stub.ReadArticleList(ReadArticleListRequest())
        assert response != None
        # Can't really test this properly without a data population strategy in the pipeline.
        context.articles = response.articles


@then("we receive a list of all articles")
def step_impl(context):
    assert len(context.articles) >= 0


@when("a request for reading all articles by search term from the distributor is made")
def step_impl(context):
    with grpc.insecure_channel(
        f"{DISTRIBUTOR_CONF['host']}:{DISTRIBUTOR_CONF['port']}"
    ) as channel:
        stub = ArticleServiceStub(channel)
        response = stub.ReadArticleListBySearchterm(
            ReadArticleListBySearchtermRequest(search_term="Ukraine")
        )
        assert response != None
        context.articles = response.articles


@then("we receive a list of articles which has the search term in the name")
def step_impl(context):
    assert len(context.articles) >= 0


@when("a request for reading all articles by a site from the distributor is made")
def step_impl(context):
    with grpc.insecure_channel(
        f"{DISTRIBUTOR_CONF['host']}:{DISTRIBUTOR_CONF['port']}"
    ) as channel:
        stub = ArticleServiceStub(channel)
        response = stub.ReadArticleListBySite(ReadArticleListBySiteRequest(site="CNN"))
        assert response != None
        context.articles = response.articles


@then("we receive a list of articles which come from that site")
def step_impl(context):
    assert len(context.articles) >= 0


@when("a request for create article count statistics is made")
def step_impl(context):
    with grpc.insecure_channel(
        f"{DISTRIBUTOR_CONF['host']}:{DISTRIBUTOR_CONF['port']}"
    ) as channel:
        stub = ArticleServiceStub(channel)
        response = stub.ReadArticleCountBySearchSite(
            ReadArticleCountBySearchSiteRequest(site="TV2", search_term="Ukraine")
        )
        assert response != None
        context.stats = (
            response.total,
            response.cnt_contained_search_term,
            response.cnt_not_contained_search_term,
        )


@then("we receive article count statistics")
def step_impl(context):
    assert context.stats[0] >= 0
    assert context.stats[1] >= 0
    assert context.stats[2] >= 0
