use lazy_static::*;
use serde_json::Value;

lazy_static! {
  #[allow(deprecated)]
  static ref config : Value = Value::Object();
}

