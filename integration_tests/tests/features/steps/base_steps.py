from behave import *
from integration_tests.utils.container_management import is_running
from integration_tests.connection import mongo_connection
from integration_tests.utils.config import DISTRIBUTOR_CONF, CONFIG
from proto_implementations.protogen.basic_pb2_grpc import BasicServiceStub
from proto_implementations.protogen.basic_pb2 import HealthCheckRequest
import grpc

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