/// Error for grpcurl-proxy
#[derive(Debug)]
pub struct InternalError(anyhow::Error);

impl InternalError {
    pub fn new(e: anyhow::Error) -> Self {
        Self(e)
    }
}

impl warp::reject::Reject for InternalError {}
