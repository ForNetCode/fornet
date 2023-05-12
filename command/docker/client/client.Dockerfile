# -*- mode: dockerfile -*-

# Dockerfile for netclient

# You can override this `--build-arg BASE_IMAGE=...` to use different
# version of Rust
ARG BASE_IMAGE=rust:1.65

#ARG RUNTIME_IMAGE=alpine
ARG RUNTIME_IMAGE=rust:1.65

FROM ${BASE_IMAGE} AS builder

RUN apt update && apt install -y cmake bash


#RUN  apt update &&  apt upgrade -y &&  apt install -y protobuf-compiler libprotobuf-dev

# Add our source code.
ADD protobuf /source/protobuf
ADD third /source/third
ADD client /source/client
ADD win-tun-driver /source/win-tun-driver
ADD command/docker/client/script.sh /script.sh
RUN chmod +x /script.sh && /script.sh
#RUN ls -al && cd protobuf && ls -al && cd ../
RUN --mount=type=cache,target=/usr/local/cargo/registry \
     cd /source/client && cargo build --release

FROM ${RUNTIME_IMAGE}

ENV FORNET_CONFIG=/config

RUN mkdir /config && apt-get update && apt install -y iproute2

COPY --from=builder /source/client/target/release/fornet /usr/bin
COPY --from=builder /source/client/target/release/fornet-cli /usr/bin


CMD ["fornet"]