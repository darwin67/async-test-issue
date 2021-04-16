use crate::common::convert::Convert;
use actix_web::{body::Body, dev::ResponseBody};
use serde_json::Value;

impl Convert for ResponseBody<Body> {
    fn as_str(&self) -> &str {
        match self {
            ResponseBody::Body(ref b) => match b {
                Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
                _ => panic!(),
            },
            ResponseBody::Other(ref b) => match b {
                Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
                _ => panic!(),
            },
        }
    }

    fn as_json(&self) -> Value {
        let value = self.as_str();
        serde_json::from_str(value).unwrap()
    }
}
