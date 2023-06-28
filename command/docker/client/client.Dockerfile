# -*- mode: dockerfile -*-

# Dockerfile for netclient

# You can override this `--build-arg BASE_IMAGE=...` to use different
# version of Rust
ARG BASE_IMAGE=rust:1.70-alpine

#ARG RUNTIME_IMAGE=alpine
ARG RUNTIME_IMAGE=alpine:latest

FROM ${BASE_IMAGE} AS builder

#RUN apk update && apk add bash 
RUN apk update && apk add pkgconfig musl-dev openssl-dev protobuf


#RUN  apt update &&  apt upgrade -y &&  apt install -y protobuf-compiler libprotobuf-dev

# Add our source code.
ADD protobuf /source/protobuf
ADD third /source/third
ADD client /source/client
#ADD command/docker/client/script.sh /script.sh
#RUN chmod +x /script.sh && /script.sh

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cd /source/client && cargo build --release

RUN   mv /source/client/target/release/fornet /fornet  && \
    mv /source/client/target/release/fornet-cli /fornet-cli
FROM ${RUNTIME_IMAGE}

ENV FORNET_CONFIG=/config

RUN mkdir /config && apk run --no-cache libc6-compat

COPY --from=builder /fornet /usr/bin
COPY --from=builder /fornet-cli /usr/bin


CMD ["fornet"]