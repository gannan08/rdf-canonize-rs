use sha2::Digest;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MessageDigest<T: Clone + Digest> {
  hasher: T,
}

impl<T: Clone + Digest> MessageDigest<T> {
  pub fn new() -> MessageDigest<T> {
    MessageDigest { hasher: T::new() }
  }

  pub fn update(&mut self, msg: &str) {
    self.hasher.update(msg);
  }

  pub fn digest(&mut self) -> String {
    hex::encode(self.hasher.clone().finalize())
  }
}
