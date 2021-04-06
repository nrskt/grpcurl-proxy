use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::anychar,
    combinator::{map, map_res},
    multi::many0,
    IResult,
};
use serde_json::Value;

use crate::Code;

/// grpcurl error message
#[derive(Clone, Debug)]
pub struct ErrorMessage {
    code: Code,
    message: Value,
}

impl ErrorMessage {
    pub fn code(&self) -> Code {
        self.code
    }

    pub fn message(&self) -> &Value {
        &self.message
    }
}

impl FromStr for ErrorMessage {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match grpcurl_err_msg(s) {
            Err(e) => anyhow::bail!("parse error, {}", e),
            Ok((_, err)) => Ok(err),
        }
    }
}

/// Parser for grpcurl error message
fn grpcurl_err_msg(input: &str) -> IResult<&str, ErrorMessage> {
    let (input, _) = tag("ERROR:  ")(input)?;
    let (input, code) = map_res(error_code, |code| code.parse())(input)?;
    let (input, _) = tag("\n  ")(input)?;
    let (input, message) = message(input)?;
    Ok((input, ErrorMessage { code, message }))
}

fn error_code(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("Code: ")(input)?;
    let (input, code) = take_until("\n")(input)?;
    Ok((input, code.to_string()))
}

fn message(input: &str) -> IResult<&str, Value> {
    let (input, _) = tag("Message: ")(input)?;
    let (input, m) = map(
        many0(anychar),
        |a| serde_json::json!({"message": a.into_iter().collect::<String>()}),
    )(input)?;
    Ok((input, m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        let str = "Message: Invalid Argument:\n XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
        let result = message(str);
        println!("{:#?}", result);
        assert!(result.is_ok())
    }

    #[test]
    fn test_grpcurl_err_msg() {
        let message = "ERROR:  Code: InvalidArgument\n  Message: Invalid Argument:\n XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
        let result = grpcurl_err_msg(message);
        println!("{:#?}", result);
        assert!(result.is_ok())
    }
}
