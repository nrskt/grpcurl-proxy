FROM rust:1.51 as builder
WORKDIR /usr/src/grpcurl-proxy
COPY . .
RUN cargo build --release

FROM ubuntu:latest
WORKDIR /usr/src
RUN apt-get update && apt-get install -y wget
RUN wget https://github.com/fullstorydev/grpcurl/releases/download/v1.8.0/grpcurl_1.8.0_linux_x86_64.tar.gz
RUN tar xvf grpcurl_1.8.0_linux_x86_64.tar.gz
RUN cp ./grpcurl /usr/local/bin/grpcurl

COPY --from=builder /usr/src/grpcurl-proxy/target/release/grpcurl-proxy /usr/local/bin/grpcurl-proxy

EXPOSE 50050
CMD ["grpcurl-proxy"]
