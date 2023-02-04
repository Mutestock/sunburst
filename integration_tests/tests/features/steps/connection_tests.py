from behave import * 
from integration_tests.utils.config import CONFIG, get_environment_specific_database_config
from integration_tests.utils.container_management import is_running
from integration_tests.connection import mongo_connection

@given('mongodb is up and running')
def step_impl(_):
    assert is_running(CONFIG["database"]["container_name"])
    assert mongo_connection.get_mongo_connection().admin.command('ismaster') != None


@when('a connection to the mongodb testing database is established')
def step_impl(context):
    db = mongo_connection.get_mongo_test_handle()
    context.test_db = db
    
    
@then("we're capable of basic interactions with the testing database")
def step_impl(context):
    assert context.test_db.command("buildinfo") != None
    
    

@when('a connection to the mongodb dev database is established')
def step_impl(context):
    db = mongo_connection.get_mongo_dev_handle()
    context.dev_db = db
    
    
@then("we're capable of basic interactions with the dev database")
def step_impl(context):
    assert context.dev_db.command("buildinfo") != None
    
    

@when('a connection to the mongodb production database is established')
def step_impl(context):
    db = mongo_connection.get_mongo_prod_handle()
    context.prod_db = db
    
    
@then("we're capable of basic interactions with the production database")
def step_impl(context):
    assert context.prod_db.command("buildinfo") != None
    
    
