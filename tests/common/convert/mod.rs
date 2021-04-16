pub(crate) mod body;

use serde_json::Value;

pub trait Convert {
    fn as_str(&self) -> &str;
    fn as_json(&self) -> Value;
}
