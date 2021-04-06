use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:50050".parse().unwrap();
    warp::serve(grpcurl_proxy::filters::proxy()).run(addr).await;
}
