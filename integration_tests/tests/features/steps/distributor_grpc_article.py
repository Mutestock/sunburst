from behave import *
from integration_tests.utils.container_management import is_running
from integration_tests.utils.config import CONFIG
from integration_tests.connection import mongo_connection


@given("mongodb and the distributor are both online")
def step_impl(_):
    assert is_running(CONFIG["database"]["container_name"])
    assert mongo_connection.get_mongo_connection().admin.command("ismaster") != None


@when("a request for reading all articles from the distributor is made")
def step_impl(context):
    pass


@then("we receive a list of all articles")
def step_impl(context):
    pass


@when("a request for reading all articles by search term from the distributor is made")
def step_impl(context):
    pass


@then("we receive a list of articles which has the search term in the name")
def step_impl(context):
    pass


@when("a request for reading all articles by a site from the distributor is made")
def step_impl(context):
    pass


@then("we receive a list of articles which come from that site")
def step_impl(context):
    pass
