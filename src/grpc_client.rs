use std::collections::HashMap;
use std::net::SocketAddr;

use async_trait::async_trait;
use serde_json::Value;
use tokio::process::Command;

use crate::{Code, ErrorMessage};

/// GrpcClient
///
/// Supports unary call only
#[async_trait]
pub trait GrpcClient {
    /// Call grpc method
    async fn unary_call(&self) -> anyhow::Result<Message>;
}

/// gRPC response message using grpcurl
#[derive(Clone, Debug)]
pub struct Message {
    code: Code,
    body: Value,
}

impl Message {
    pub fn body(&self) -> &Value {
        &self.body
    }
}

impl From<ErrorMessage> for Message {
    fn from(value: ErrorMessage) -> Self {
        let msg = Message {
            code: value.code(),
            body: value.message().clone(),
        };
        msg
    }
}

pub struct LocalGrpcurlCommand {
    /// If tls is false then use `-plaintext` option
    tls: bool,
    /// Destination address
    addr: SocketAddr,
    /// gRPC method name
    /// format: `<Service name>/<Method name>`
    method: String,
    /// gRPC request message
    message: Value,
    /// gRPC request metadata
    headers: HashMap<HeaderKey, HeaderValue>,
}

type HeaderKey = String;
type HeaderValue = String;

impl LocalGrpcurlCommand {
    pub fn new(
        tls: bool,
        addr: SocketAddr,
        method: String,
        message: Value,
        headers: HashMap<HeaderKey, HeaderValue>,
    ) -> Self {
        Self {
            tls,
            addr,
            method,
            message,
            headers,
        }
    }
}

#[async_trait]
impl GrpcClient for LocalGrpcurlCommand {
    async fn unary_call(&self) -> anyhow::Result<Message> {
        let mut cmd = Command::new("grpcurl");
        if !self.tls {
            cmd.arg("-plaintext");
        }
        for (k, v) in self.headers.iter() {
            cmd.arg("-rpc-header").arg(format!("{}: {}", k, v));
        }
        let output = cmd
            .arg("-d")
            .arg(format!("{}", self.message))
            .arg(format!("{}", self.addr))
            .arg(self.method.to_string())
            .output()
            .await?;
        if output.status.code() != Some(0) {
            let err_msg: ErrorMessage = String::from_utf8(output.stderr)?.parse()?;
            return Ok(err_msg.into());
        }
        let msg = serde_json::from_slice(output.stdout.as_slice())?;
        Ok(Message {
            code: Code::Ok,
            body: msg,
        })
    }
}
