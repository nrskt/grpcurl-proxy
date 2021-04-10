use std::env;
use std::net::SocketAddr;

use grpcurl_proxy::Application;

#[tokio::main]
async fn main() {
    env_logger::init();
    let app = match env::var("DESTINATION_ADDR")
        .map_err(|e| anyhow::anyhow!(e))
        .and_then(|dest: String| Application::new(&dest))
    {
        Err(e) => {
            log::error!("Error: {}", e);
            std::process::exit(1)
        }
        Ok(r) => r,
    };
    let addr: SocketAddr = "0.0.0.0:50050".parse().unwrap();
    warp::serve(grpcurl_proxy::filters::proxy(app))
        .run(addr)
        .await;
}
