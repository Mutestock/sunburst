from behave import *
import grpc
from proto_implementations.protogen.basic_pb2 import HealthCheckRequest
from integration_tests.utils.config import DISTRIBUTOR_CONF, CONFIG, RS_REST_CONF
from integration_tests.utils.container_management import is_running
from integration_tests.connection import mongo_connection
from proto_implementations.protogen.basic_pb2_grpc import BasicServiceStub
import requests


@given("mongodb is online")
def step_impl(_):
    assert is_running(CONFIG["database"]["container_name"])
    assert mongo_connection.get_mongo_connection().admin.command("ismaster") != None


@given("the distributor is online")
def step_impl(_):
    with grpc.insecure_channel(
        f"{DISTRIBUTOR_CONF['host']}:{DISTRIBUTOR_CONF['port']}"
    ) as channel:
        stub = BasicServiceStub(channel)
        response = stub.HealthCheck(HealthCheckRequest())
        assert response.msg == "Ok"


@given("the rest service is online")
def step_impl(_):
    r = requests.get(f"http://{RS_REST_CONF['host']}:{RS_REST_CONF['port']}/health")
    assert r.status_code == 200


@when("a request for reading all articles from the rust rest service is made")
def step_impl(context):
    r = requests.get(f"http://{RS_REST_CONF['host']}:{RS_REST_CONF['port']}/article")
    assert r.status_code == 200
    context.articles = r.json()


@then("we receive a list of all articles from the rust rest service")
def step_impl(context):
    assert context.articles is not None
    assert len(context.articles) >= 0


@when(
    "a request for reading all articles by search term from the rust rest service is made"
)
def step_impl(context):
    r = requests.get(
        f"http://{RS_REST_CONF['host']}:{RS_REST_CONF['port']}/article/search-term=something"
    )
    assert r.status_code == 200
    context.articles = r.json()


@then(
    "we receive a list of all articles which has the search term in the name or tags from the rust rest service"
)
def step_impl(context):
    assert context.articles is not None
    assert len(context.articles) >= 0


@when("a request for reading all articles by site from the rust rest service is made")
def step_impl(context):
    r = requests.get(f"http://{RS_REST_CONF['host']}:{RS_REST_CONF['port']}/article/site=TV2")
    assert r.status_code == 200
    context.articles = r.json()


@then(
    "we receive a list of all articles which come from that site from the rust rest service"
)
def step_impl(context):
    assert context.articles is not None
    assert len(context.articles) >= 0


@when('A request for reading article count statistics from the rust rest service is made')
def step_impl(context):
    r = requests.get(f"http://{RS_REST_CONF['host']}:{RS_REST_CONF['port']}/article/count/site=TV2/search=thing")
    assert r.status_code == 200
    context.article_count = r.json()
    
    
@then("we receive article count statistics from the rust rest service")
def step_impl(context):
    
    assert context.article_count["total"] >= 0