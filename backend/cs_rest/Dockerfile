FROM bitnami/dotnet-sdk:latest

COPY backend/cs_rest /backend/cs_rest
COPY config.toml /config.toml
COPY proto /proto

WORKDIR /backend/cs_rest

CMD [ "dotnet", "run", "--launch-profile", "containerized"]
