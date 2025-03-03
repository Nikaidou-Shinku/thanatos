use std::{
  process::Command,
  time::{SystemTime, UNIX_EPOCH},
};

fn main() {
  let output = Command::new("git")
    .args(["rev-parse", "HEAD"])
    .output()
    .expect("failed to get the git commit hash");
  let hash = String::from_utf8(output.stdout).expect("failed to parse `git rev-parse HEAD`");

  println!("cargo:rustc-env=GIT_HASH={hash}");
  println!("cargo:rerun-if-changed=.git/HEAD");

  // SAFETY: `SystemTime::now()` is always later than `UNIX_EPOCH`.
  let timestamp = unsafe {
    SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_unchecked()
      .as_millis()
  };

  println!("cargo:rustc-env=BUILD_TIME={timestamp}");
}
