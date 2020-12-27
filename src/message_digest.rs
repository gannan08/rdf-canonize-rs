use ring::digest;

#[derive(Clone)]
pub struct MessageDigest {
  ctx: digest::Context,
}

impl MessageDigest {
  pub fn new(algorithm: &'static digest::Algorithm) -> MessageDigest {
    MessageDigest {
      ctx: digest::Context::new(algorithm),
    }
  }

  pub fn update(&mut self, msg: &[u8]) {
    self.ctx.update(msg);
  }

  pub fn digest(md: MessageDigest) -> String {
    hex::encode(md.ctx.finish())
  }
}
