FROM rust:1.67.1-buster as BUILDER
COPY distributor /distributor
COPY commons /commons
COPY config.toml /config.toml
COPY proto /proto
ENV PB_REL="https://github.com/protocolbuffers/protobuf/releases"
ENV ZIP_FILE="protoc-21.0-linux-x86_64.zip"
RUN curl -LO $PB_REL/download/v21.0/$ZIP_FILE
RUN unzip $ZIP_FILE -d /protoc
ENV PROTOC="/protoc/bin/protoc"
WORKDIR /distributor

RUN cargo b --release


FROM debian:buster-slim
COPY --from=BUILDER /distributor/target/release/distributor distributor
# This config file will have security issues if it is included in published images.
COPY --from=BUILDER /config.toml /config.toml

CMD [ "./distributor" ]
