use std::collections::HashMap;
use std::convert::TryInto;

use http::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;

use crate::{Application, GrpcClient, InternalError, LocalGrpcurlCommand};

/// Handler for calling grpc method
pub(crate) async fn proxy_handler(
    app: Application,
    service: String,
    method: String,
    message: Value,
    headers: HeaderMap,
) -> Result<impl warp::Reply, warp::Rejection> {
    let map = extract_header(headers);

    let cmd = LocalGrpcurlCommand::new(
        false,
        app.dest(),
        format!("{}/{}", service, method),
        message,
        map,
    );
    match cmd.unary_call().await {
        Err(e) => {
            println!("Failed to call method");
            Err(warp::reject::custom(InternalError::new(e)))
        }
        Ok(r) => Ok(warp::reply::json(r.body())),
    }
}

/// Extract the headers which have the key started "x-"
fn extract_header(header: HeaderMap) -> HashMap<String, String> {
    header
        .into_iter()
        .filter_map(|(k, v)| k.map(|k| (k, v)))
        .flat_map(|(k, v)| filter_prefix(k, v))
        .collect::<HashMap<_, _>>()
}

fn filter_prefix(key: HeaderName, value: HeaderValue) -> anyhow::Result<(String, String)> {
    let header_key: [&str; 2] = key
        .as_str()
        .split("x-")
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;

    match header_key {
        ["", key] => Ok((key.to_string(), value.to_str()?.to_string())),
        _ => anyhow::bail!("Ignore"),
    }
}
