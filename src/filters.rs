use warp::Filter;

use super::handlers;

/// Filter for grpcurl proxy
pub fn proxy() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String / String)
        .and(warp::body::json())
        .and(warp::header::headers_cloned())
        .and(warp::post())
        .and_then(handlers::proxy_handler)
}
