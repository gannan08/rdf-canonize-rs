#[macro_use]
extern crate lazy_static;

mod identifier_issuer;
mod message_digest;
mod permuter;
mod permuter_fast;
mod urdna2015;

pub mod nquads;

pub fn canonize(dataset: &nquads::Dataset, algorithm: &str) -> Option<String> {
  match algorithm {
    "URDNA2015" => Some(urdna2015::URDNA2015::new().main(&dataset)),
    _ => None,
  }
}
