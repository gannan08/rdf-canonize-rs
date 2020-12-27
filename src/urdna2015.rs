use crate::identifier_issuer::IdentifierIssuer;
use crate::message_digest::MessageDigest;
use crate::nquads;
use crate::nquads::{Dataset, Quad, QuadSet, Term, TermType};
use crate::permuter::Permuter;

use lexical_sort::natural_lexical_cmp;
use sha2::Sha256;
use std::collections::HashMap;

const NAME: &str = "URDNA2015";
const HASH_ALGORITHM: &str = "sha256";

type Hash = String;
type BlankNodeInfoMap = HashMap<String, BlankNodeInfo>;
type HashBlankNodeMap = HashMap<String, Vec<String>>;
type HashToRelatedMap = HashMap<String, Vec<String>>;

#[derive(Clone, Debug, PartialEq)]
struct BlankNodeInfo {
  pub quads: QuadSet,
  hash: Option<Hash>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct URDNA2015 {
  name: String,
  blank_node_info: BlankNodeInfoMap,
  canonical_issuer: IdentifierIssuer,
  hash_algorithm: String,
  quads: QuadSet,
}

impl URDNA2015 {
  pub fn new() -> URDNA2015 {
    URDNA2015 {
      name: String::from(NAME),
      blank_node_info: BlankNodeInfoMap::new(),
      canonical_issuer: IdentifierIssuer::new("_:c14n"),
      hash_algorithm: String::from(HASH_ALGORITHM),
      quads: vec![],
    }
  }

  // 4.4) Normalization Algorithm
  pub fn main(&mut self, dataset: &Dataset) -> String {
    self.quads = dataset.quads.clone();
    let quads = &dataset.quads;
    // 1) Create the normalization state.

    // 2) For every quad in input dataset:
    for quad in quads {
      // 2.1) For each blank node that occurs in the quad, add a reference
      // to the quad using the blank node identifier in the blank node to
      // quads map, creating a new entry if necessary.
      self.add_blank_node_quad_info(&quad, &quad.subject);
      self.add_blank_node_quad_info(&quad, &quad.object);
      self.add_blank_node_quad_info(&quad, &quad.graph);
    }

    // 3) Create a list of non-normalized blank node identifiers
    // non-normalized identifiers and populate it using the keys from the
    // blank node to quads map.
    // Note: We use a map here and it was generated during step 2.

    // 4) `simple` flag is skipped -- loop is optimized away. This optimization
    // is permitted because there was a typo in the hash first degree quads
    // algorithm in the URDNA2015 spec that was implemented widely making it
    // such that it could not be fixed; the result was that the loop only
    // needs to be run once and the first degree quad hashes will never change.
    // 5.1-5.2 are skipped; first degree quad hashes are generated just once
    // for all non-normalized blank nodes.

    // 5.3) For each blank node identifier identifier in non-normalized
    // identifiers:
    let mut hash_to_blank_nodes = HashBlankNodeMap::new();
    let non_normalized = hashmap_keys_to_vec(&self.blank_node_info);
    for id in &non_normalized {
      self.hash_and_track_blank_node(id, &mut hash_to_blank_nodes)
    }

    // 5.4) For each hash to identifier list mapping in hash to blank
    // nodes map, lexicographically-sorted by hash:
    let mut hashes = hashmap_keys_to_vec(&hash_to_blank_nodes);
    hashes.sort_unstable();
    // optimize away second sort, gather non-unique hashes in order as we go
    let mut non_unique: Vec<Vec<String>> = Vec::new();

    for hash in &hashes {
      // 5.4.1) If the length of identifier list is greater than 1,
      // continue to the next mapping.
      let mut id_list = hash_to_blank_nodes.get(hash).unwrap().clone();
      id_list.sort_by(|a, b| natural_lexical_cmp(&a, &b));
      if id_list.len() > 1 {
        non_unique.push(id_list);
        continue;
      }

      // 5.4.2) Use the Issue Identifier algorithm, passing canonical
      // issuer and the single blank node identifier in identifier
      // list, identifier, to issue a canonical replacement identifier
      // for identifier.
      let id = &id_list[0];
      self.canonical_issuer.get_id(id.to_string());

      // Note: These steps are skipped, optimized away since the loop
      // only needs to be run once.
      // 5.4.3) Remove identifier from non-normalized identifiers.
      // 5.4.4) Remove hash from the hash to blank nodes map.
      // 5.4.5) Set simple to true.
    }

    // 6) For each hash to identifier list mapping in hash to blank nodes map,
    // lexicographically-sorted by hash:
    // Note: sort optimized away, use `non_unique`.
    for id_list in &mut non_unique {
      // 6.1) Create hash path list where each item will be a result of
      // running the Hash N-Degree Quads algorithm.
      let mut hash_path_list = vec![];
      // 6.2) For each blank node identifier identifier in identifier list:
      for id in id_list {
        // 6.2.1) If a canonical identifier has already been issued for
        // identifier, continue to the next identifier.
        if self.canonical_issuer.has_id(&id) {
          continue;
        }

        // 6.2.2) Create temporary issuer, an identifier issuer
        // initialized with the prefix _:b.
        let mut issuer = IdentifierIssuer::new("_:b");

        // 6.2.3) Use the Issue Identifier algorithm, passing temporary
        // issuer and identifier, to issue a new temporary blank node
        // identifier for identifier.
        issuer.get_id(id.to_string());

        // 6.2.4) Run the Hash N-Degree Quads algorithm, passing
        // temporary issuer, and append the result to the hash path list.
        let result = self.hash_n_degree_quads(&id, &mut issuer);
        hash_path_list.push(result);
      }

      // 6.3) For each result in the hash path list,
      // lexicographically-sorted by the hash in result:
      hash_path_list.sort_by(|a, b| natural_lexical_cmp(&a.0, &b.0));
      for result in hash_path_list {
        // 6.3.1) For each blank node identifier, existing identifier,
        // that was issued a temporary identifier by identifier issuer
        // in result, issue a canonical identifier, in the same order,
        // using the Issue Identifier algorithm, passing canonical
        // issuer and existing identifier.
        let old_ids = result.1.get_old_ids();
        for id in old_ids {
          self.canonical_issuer.get_id(id.to_string());
        }
      }
    }

    /* Note: At this point all blank nodes in the set of RDF quads have been
    assigned canonical identifiers, which have been stored in the canonical
    issuer. Here each quad is updated by assigning each of its blank nodes
    its new identifier. */

    // 7) For each quad, quad, in input dataset:
    let mut normalized = vec![];
    for quad in self.quads.iter() {
      // 7.1) Create a copy, quad copy, of quad and replace any existing
      // blank node identifiers using the canonical identifiers
      // previously issued by canonical issuer.
      let mut q = quad.clone();
      q.subject = Self::use_canonical_id(&mut q.subject, &mut self.canonical_issuer);
      q.object = Self::use_canonical_id(&mut q.object, &mut self.canonical_issuer);
      q.graph = Self::use_canonical_id(&mut q.graph, &mut self.canonical_issuer);
      // 7.2) Add quad copy to the normalized dataset.
      normalized.push(nquads::serialize_quad(&q));
    }

    // sort normalized output
    normalized.sort_unstable();

    // 8) Return the normalized dataset.
    normalized.join("")
  }

  // 4.6) Hash First Degree Quads
  fn hash_first_degree_quads(&mut self, id: &str) -> String {
    // 1) Initialize nquads to an empty list. It will be used to store quads in
    // N-Quads format.
    let mut serialized_quads: Vec<String> = Vec::new();

    // 2) Get the list of quads `quads` associated with the reference blank node
    // identifier in the blank node to quads map.
    let mut info = self.blank_node_info.get_mut(id).unwrap();

    for quad in &mut info.quads {
      // 3.1) Serialize the quad in N-Quads format with the following special
      // rule:

      // 3.1.1) If any component in quad is an blank node, then serialize it
      // using a special identifier as follows:
      let mut copy = Quad::new();
      copy.predicate = quad.predicate.clone();
      // 3.1.2) If the blank node's existing blank node identifier matches
      // the reference blank node identifier then use the blank node
      // identifier _:a, otherwise, use the blank node identifier _:z.
      copy.subject = Self::modify_first_degree_component(id, &mut quad.subject).clone();
      copy.object = Self::modify_first_degree_component(id, &mut quad.object).clone();
      copy.graph = Self::modify_first_degree_component(id, &mut quad.graph).clone();
      serialized_quads.push(nquads::serialize_quad(&copy));
    }

    // 4) Sort nquads in lexicographical order.
    serialized_quads.sort_by(|a, b| natural_lexical_cmp(&a, &b));

    // 5) Return the hash that results from passing the sorted, joined nquads
    // through the hash algorithm.
    let mut md: MessageDigest<Sha256> = MessageDigest::new();
    for quad in &serialized_quads {
      md.update(&quad);
    }
    let hex = md.digest();
    info.hash = Some(hex.clone());

    hex
  }

  // 4.7) Hash Related Blank Node
  fn hash_related_blank_node(
    &mut self,
    related: &str,
    quad: &Quad,
    issuer: &mut IdentifierIssuer,
    position: &str,
  ) -> String {
    // 1) Set the identifier to use for related, preferring first the canonical
    // identifier for related if issued, second the identifier issued by issuer
    // if issued, and last, if necessary, the result of the Hash First Degree
    // Quads algorithm, passing related.
    let id;
    if self.canonical_issuer.has_id(related) {
      id = self.canonical_issuer.get_id(related.to_string());
    } else if issuer.has_id(related) {
      id = issuer.get_id(related.to_string());
    } else if let Some(info) = self.blank_node_info.get(related) {
      id = info.hash.as_ref().unwrap().to_string();
    } else {
      id = "".to_string();
    }

    // 2) Initialize a string input to the value of position.
    // Note: We use a hash object instead.
    let mut md: MessageDigest<Sha256> = MessageDigest::new();
    md.update(position);

    // 3) If position is not g, append <, the value of the predicate in quad,
    // and > to input.
    if position != "g" {
      md.update(&self.get_related_predicate(quad));
    }

    // 4) Append identifier to input.
    md.update(&id);

    // 5) Return the hash that results from passing input through the hash
    // algorithm.
    md.digest()
  }

  // 4.8) Hash N-Degree Quads
  fn hash_n_degree_quads(
    &mut self,
    id: &str,
    issuer: &mut IdentifierIssuer,
  ) -> (String, IdentifierIssuer) {
    // 1) Create a hash to related blank nodes map for storing hashes that
    // identify related blank nodes.
    // Note: 2) and 3) handled within `create_hash_to_related`
    let mut md: MessageDigest<Sha256> = MessageDigest::new();
    let mut hash_to_related = self.create_hash_to_related(id, &mut issuer.clone());

    // 4) Create an empty string, data to hash.
    // Note: We created a hash object `md` above instead.

    // 5) For each related hash to blank node list mapping in hash to related
    // blank nodes map, sorted lexicographically by related hash:
    let mut hashes = hashmap_keys_to_vec(&hash_to_related);
    hashes.sort_unstable();
    for hash in hashes {
      // 5.1) Append the related hash to the data to hash.
      md.update(&hash);

      // 5.2) Create a string chosen path.
      let mut chosen_path = String::from("");
      // 5.3) Create an unset chosen issuer variable.
      let mut chosen_issuer: IdentifierIssuer = IdentifierIssuer::default();

      // 5.4) For each permutation of blank node list:
      let l = hash_to_related.get_mut(&hash).unwrap();
      let mut list = vec![];
      for a in l {
        list.push(&a[..]);
      }
      let mut elements = Permuter::elements(&mut list);
      let mut element_refs = Vec::with_capacity(elements.len());
      for element in elements.iter_mut() {
        element_refs.push(element);
      }
      let permuter = Permuter::new(&mut element_refs);
      for permutation in permuter {
        // 5.4.1) Create a copy of issuer, issuer copy.
        let mut issuer_copy = issuer.clone();

        // 5.4.2) Create a string path.
        let mut path = vec![];

        // 5.4.3) Create a recursion list, to store blank node identifiers
        // that must be recursively processed by this algorithm.
        let mut recursion_list: Vec<String> = vec![];

        // 5.4.4) For each related in permutation:
        let mut next_permutation = false;
        for related in permutation.iter() {
          // 5.4.4.1) If a canonical identifier has been issued for
          // related, append it to path.
          if self.canonical_issuer.has_id(&related) {
            path.push(self.canonical_issuer.get_id(related.to_string()))
          } else {
            // 5.4.4.2) Otherwise:
            // 5.4.4.2.1) If issuer copy has not issued an identifier for
            // related, append related to recursion list.
            if !issuer_copy.has_id(&related) {
              recursion_list.push(related.to_string())
            }
            // 5.4.4.2.2) Use the Issue Identifier algorithm, passing
            // issuer copy and related and append the result to path.
            path.push(issuer_copy.get_id(related.to_string()))
          }

          // 5.4.4.3) If chosen path is not empty and the length of path
          // is greater than or equal to the length of chosen path and
          // path is lexicographically greater than chosen path, then
          // skip to the next permutation.
          // Note: Comparing path length to chosen path length can be optimized
          // away; only compare lexicographically.
          if chosen_path.is_empty() && path.join("") < chosen_path {
            next_permutation = true;
            break;
          }
        }

        if next_permutation {
          continue;
        }

        // 5.4.5) For each related in recursion list:
        for related in recursion_list.iter() {
          // 5.4.5.1) Set result to the result of recursively executing
          // the Hash N-Degree Quads algorithm, passing related for
          // identifier and issuer copy for path identifier issuer.
          let result = self.hash_n_degree_quads(related, &mut issuer_copy);
          // copy and related and append the result to path.
          path.push(issuer_copy.get_id(related.to_string()));

          // 5.4.5.3) Append <, the hash in result, and > to path.
          path.push(format!("<{}>", result.0));

          // 5.4.5.4) Set issuer copy to the identifier issuer in
          // result.
          issuer_copy = result.1.clone();

          // 5.4.5.5) If chosen path is not empty and the length of path
          // is greater than or equal to the length of chosen path and
          // path is lexicographically greater than chosen path, then
          // skip to the next permutation.
          // Note: Comparing path length to chosen path length can be optimized
          // away; only compare lexicographically.
          if chosen_path.is_empty() && path.join("") < chosen_path {
            next_permutation = true;
            break;
          }
        }

        if next_permutation {
          continue;
        }

        // 5.4.6) If chosen path is empty or path is lexicographically
        // less than chosen path, set chosen path to path and chosen
        // issuer to issuer copy.
        let path_str = path.join("");
        if chosen_path.is_empty() || path_str < chosen_path {
          chosen_path = path_str;
          chosen_issuer = issuer_copy;
        }
      }

      // 5.5) Append chosen path to data to hash.
      md.update(&chosen_path);

      // 5.6) Replace issuer, by reference, with chosen issuer.
      issuer.prefix = chosen_issuer.prefix.clone();
      issuer.counter = chosen_issuer.counter;
      issuer.existing = chosen_issuer.existing.clone();
      issuer.old_ids = chosen_issuer.old_ids.clone();
    }

    (md.digest(), issuer.clone())
  }

  // helper for modifying component during Hash First Degree Quads
  fn modify_first_degree_component<'a, T>(id: &str, component: &'a mut T) -> T
  where
    T: Term + Clone,
  {
    let mut c = component.clone();
    if c.get_term_type() != TermType::BlankNode {
      return c;
    }

    let value = if c.get_value() == id { "_:a" } else { "_:z" };
    c.set_value(&String::from(value));
    c.set_term_type(&TermType::BlankNode);

    c
  }

  // helper for getting a related predicate
  fn get_related_predicate(&self, quad: &Quad) -> String {
    format!("<{}>", quad.predicate.get_value())
  }

  // helper for creating hash to related blank nodes map
  fn create_hash_to_related(
    &mut self,
    id: &str,
    issuer: &mut IdentifierIssuer,
  ) -> HashToRelatedMap {
    // 1) Create a hash to related blank nodes map for storing hashes that
    // identify related blank nodes.
    let mut hash_to_related = HashToRelatedMap::new();

    // 2) Get a reference, quads, to the list of quads in the blank node to
    // quads map for the key identifier.
    let quads = self.blank_node_info.get_mut(id).unwrap().quads.clone();

    // 3) For each quad in quads:
    for quad in quads {
      // 3.1) For each component in quad, if component is the subject, object,
      // or graph name and it is a blank node that is not identified by
      // identifier:
      // steps 3.1.1 and 3.1.2 occur in helpers:
      self.add_related_blank_node_hash(&quad, &quad.subject, "s", id, issuer, &mut hash_to_related);
      self.add_related_blank_node_hash(&quad, &quad.object, "o", id, issuer, &mut hash_to_related);
      self.add_related_blank_node_hash(&quad, &quad.graph, "g", id, issuer, &mut hash_to_related);
    }

    hash_to_related
  }

  fn hash_and_track_blank_node(&mut self, id: &str, hash_to_blank_nodes: &mut HashBlankNodeMap) {
    // 5.3.1) Create a hash, hash, according to the Hash First Degree
    // Quads algorithm.
    let hash = self.hash_first_degree_quads(id);

    // 5.3.2) Add hash and identifier to hash to blank nodes map,
    // creating a new entry if necessary.
    match hash_to_blank_nodes.get_mut(&hash) {
      Some(id_list) => id_list.push(id.to_string()),
      None => {
        let ids = vec![id.to_string()];
        hash_to_blank_nodes.insert(hash, ids);
      }
    }
  }

  fn add_blank_node_quad_info<T>(&mut self, quad: &Quad, component: &T)
  where
    T: Term,
  {
    if component.get_term_type() != TermType::BlankNode {
      return;
    }

    let id = component.get_value();
    if let Some(info) = self.blank_node_info.get_mut(&id) {
      info.quads.push(quad.clone());
    } else {
      let mut quads = QuadSet::new();
      quads.push(quad.clone());
      self
        .blank_node_info
        .insert(id, BlankNodeInfo { quads, hash: None });
    }
  }

  fn add_related_blank_node_hash<'a, T>(
    &mut self,
    quad: &Quad,
    component: &'a T,
    position: &str,
    id: &str,
    issuer: &mut IdentifierIssuer,
    hash_to_related: &'a mut HashBlankNodeMap,
  ) where
    T: Term,
  {
    let related = component.get_value();
    if !(component.get_term_type() == TermType::BlankNode && related != id) {
      return;
    }
    // 3.1.1) Set hash to the result of the Hash Related Blank Node
    // algorithm, passing the blank node identifier for component as
    // related, quad, path identifier issuer as issuer, and position as
    // either s, o, or g based on whether component is a subject, object,
    // graph name, respectively.
    let hash = self.hash_related_blank_node(&related, quad, issuer, position);

    // 3.1.2) Add a mapping of hash to the blank node identifier for
    // component to hash to related blank nodes map, adding an entry as
    // necessary.
    if let Some(entries) = hash_to_related.get_mut(&hash) {
      entries.push(related);
    } else {
      hash_to_related.insert(hash, vec![related]);
    }
  }

  fn use_canonical_id<'a, T>(component: &'a mut T, issuer: &mut IdentifierIssuer) -> T
  where
    T: Term + Clone,
  {
    let mut c = component.clone();
    if c.get_term_type() == TermType::BlankNode
      && !component.get_value().starts_with(&issuer.prefix)
    {
      c.set_value(&issuer.get_id(c.get_value()));
      return c;
    }

    c
  }
}

fn hashmap_keys_to_vec<T: Clone, U>(hashmap: &HashMap<T, U>) -> Vec<T> {
  hashmap.keys().cloned().collect()
}
