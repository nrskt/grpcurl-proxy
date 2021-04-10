use warp::Filter;

use super::{handlers, Application};

/// Filter for grpcurl proxy
pub fn proxy(
    app: Application,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String / String)
        .and(warp::body::json())
        .and(warp::header::headers_cloned())
        .and(warp::post())
        .and_then(move |svr, mtd, msg, hdr| {
            handlers::proxy_handler(app.clone(), svr, mtd, msg, hdr)
        })
}
