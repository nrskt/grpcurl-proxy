# grpcurl-proxy

grpcurl-proxy is a proxy server for http to gRPC. It makes enables to call the gRPC method using Postman(like tools). 

## Requirements

- gRPC server which supported reflection.

## How to use

run grpcurl-proxy with docker

```
docker run \
  -d \
  --rm \
  --add-host=host.docker.internal:host-gateway \
  -p "50050:50050" \
  -e DESTINATION_ADDR=host.docker.internal:50051 \
  grpcurl-proxy
```

