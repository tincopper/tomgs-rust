use std::fmt::Debug;
use std::any::Any;

fn load_config<T: Debug + Any>(value: &T) -> Vec<String> {
  let mut cfg = vec![];
  let value = value as &dyn Any;
  match value.downcast_ref::<String>() {
    Some(cfp) => cfg.push(cfp.clone()),
    None => {}
  };

  match value.downcast_ref::<Vec<String>>() {
    Some(cfp) => cfg.extend_from_slice(&cfp),
    None => {}
  };

  cfg
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_reflect() {
    let cfp = "/etc/wayslog.conf".to_string();
    assert_eq!(load_config(&cfp), vec!["/etc/wayslog.conf".to_string()]);

    let cfps = vec!["/etc/wayslog.conf".to_string(), "/etc/wayslog_sec.conf".to_string()];
    assert_eq!(load_config(&cfps), vec!["/etc/wayslog.conf".to_string(), "/etc/wayslog_sec.conf".to_string()]);
  }

}