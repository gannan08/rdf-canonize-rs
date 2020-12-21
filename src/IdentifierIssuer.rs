use std::collections::HashMap;

pub type NodeIdentifier = String;
pub type NodeIdentifierList = Vec<NodeIdentifier>;
pub type NodeIdentifierMap = HashMap<NodeIdentifier, NodeIdentifier>;

pub struct IdentifierIssuer {
  prefix: String,
  counter: u64,
  existing: NodeIdentifierMap,
  ordered: NodeIdentifierList
}

pub struct IdentifierIssuerPool {
  issuers: Vec<IdentifierIssuer*>
}
