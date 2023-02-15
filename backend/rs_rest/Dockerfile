FROM rust:latest as BUILDER

COPY backend/rs_rest /backend/rs_rest
COPY config.toml /config.toml
COPY commons /commons
COPY proto /proto
ENV PB_REL="https://github.com/protocolbuffers/protobuf/releases"
ENV ZIP_FILE="protoc-21.0-linux-x86_64.zip"
RUN curl -LO $PB_REL/download/v21.0/$ZIP_FILE
RUN unzip $ZIP_FILE -d /protoc
ENV PROTOC="/protoc/bin/protoc"
WORKDIR /backend/rs_rest

RUN cargo b --release

FROM debian:buster-slim
COPY --from=BUILDER /backend/rs_rest/target/release/rs_rest rs_rest
COPY --from=BUILDER /config.toml /config.toml

EXPOSE 22550

CMD ["./rs_rest"]
