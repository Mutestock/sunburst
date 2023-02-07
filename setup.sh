#!/bin/bash

# Protogen
rm -rf ./commons/py/proto_implementations/proto_implementations/protogen
mkdir ./commons/py/proto_implementations/proto_implementations/protogen

# Protogen
echo generating protogen
python -m grpc_tools.protoc -I./proto --python_out=./commons/py/proto_implementations/proto_implementations/protogen --grpc_python_out=./commons/py/proto_implementations/proto_implementations/protogen ./proto/article.proto

echo Ok