use std::{
  // cell::OnceCell,
  time::{SystemTime, UNIX_EPOCH},
};

use data_encoding::HEXUPPER;
use md5::{Digest, Md5};
use uuid::Uuid;

pub fn device_token() -> String {
  // TODO: fix these
  // const DEVICE_TOKEN: OnceCell<Uuid> = OnceCell::new();
  // *DEVICE_TOKEN.get_or_init(|| Uuid::new_v4())
  "3B9ECD7D741253FB3C7929796652C084".to_owned()
}

pub fn sf_security() -> String {
  let nonce = Uuid::new_v4();

  // SAFETY: `SystemTime::now()` is always later than `UNIX_EPOCH`.
  let timestamp = unsafe {
    SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_unchecked()
      .as_millis()
  };

  let device_token = device_token();

  let sign = {
    const SF_SALT: &str = "FN_Q29XHVmfV3mYX";
    let hash = Md5::digest(format!("{nonce}{timestamp}{device_token}{SF_SALT}"));
    HEXUPPER.encode(&hash)
  };

  format!("nonce={nonce}&timestamp={timestamp}&devicetoken={device_token}&sign={sign}")
}
