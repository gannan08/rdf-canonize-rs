use std::collections::HashMap;

pub type NodeIdentifier = String;
pub type NodeIdentifierMap = HashMap<NodeIdentifier, NodeIdentifier>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct IdentifierIssuer {
  pub prefix: String,
  pub counter: u64,
  pub existing: NodeIdentifierMap,
  pub old_ids: Vec<String>,
}

impl IdentifierIssuer {
  pub fn new(prefix: &str) -> IdentifierIssuer {
    let counter = 0;
    let existing = NodeIdentifierMap::new();

    IdentifierIssuer {
      prefix: prefix.to_string(),
      counter,
      existing,
      old_ids: vec![],
    }
  }

  pub fn get_id(&mut self, old: &str) -> String {
    // return existing old identifier
    if let Some(existing) = self.get_existing_id(old) {
      return existing;
    }

    // get next identifier
    let identifier = self.prefix.to_string() + &self.counter.to_string();
    self.counter += 1;

    // save mapping
    self.old_ids.push(old.to_string());
    self
      .existing
      .insert(old.to_string(), identifier.to_string());

    identifier
  }

  pub fn get_existing_id(&self, old: &str) -> Option<String> {
    // return existing old identifier
    if let Some(existing) = self.existing.get(old) {
      return Some(existing.to_string());
    }

    None
  }

  pub fn has_id(&self, old: &str) -> bool {
    self.existing.contains_key(old)
  }

  pub fn get_old_ids(&self) -> &[String] {
    &self.old_ids[..]
  }
}
