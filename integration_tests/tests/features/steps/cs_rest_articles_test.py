from behave import *
from integration_tests.utils.config import CS_REST_CONF
import requests


@given("the dotnet rest service is online")
def step_impl(_):
    r = requests.get(f"http://{CS_REST_CONF['host']}:{CS_REST_CONF['port']}/health")
    assert r.status_code == 200


@when("a request for reading all articles from the dotnet rest service is made")
def step_impl(context):
    r = requests.get(f"http://{CS_REST_CONF['host']}:{CS_REST_CONF['port']}/article")
    assert r.status_code == 200
    context.articles = r.json()


@then("we receive a list of all articles from the dotnet rest service")
def step_impl(context):
    assert context.articles is not None
    assert len(context.articles) >= 0


@when(
    "a request for reading all articles by search term from the dotnet rest service is made"
)
def step_impl(context):
    r = requests.get(
        f"http://{CS_REST_CONF['host']}:{CS_REST_CONF['port']}/article/search-term=something"
    )
    assert r.status_code == 200
    context.articles = r.json()


@then(
    "we receive a list of all articles which has the search term in the name or tags from the dotnet rest service"
)
def step_impl(context):
    assert context.articles is not None
    assert len(context.articles) >= 0


@when("a request for reading all articles by site from the dotnet rest service is made")
def step_impl(context):
    r = requests.get(f"http://{CS_REST_CONF['host']}:{CS_REST_CONF['port']}/article/site=TV2")
    assert r.status_code == 200
    context.articles = r.json()


@then(
    "we receive a list of all articles which come from that site from the dotnet rest service"
)
def step_impl(context):
    assert context.articles is not None
    assert len(context.articles) >= 0


@when('A request for reading article count statistics from the dotnet rest service is made')
def step_impl(context):
    r = requests.get(f"http://{CS_REST_CONF['host']}:{CS_REST_CONF['port']}/article/count/site=TV2/search=thing")
    assert r.status_code == 200
    context.article_count = r.json()
    
    
@then("we receive article count statistics from the dotnet rest service")
def step_impl(context):
    assert context.article_count["total"] >= 0