use crate::identifier_issuer::IdentifierIssuer;
use crate::message_digest::MessageDigest;
use crate::nquads;
use crate::nquads::{Dataset, Quad, Term, TermType};
use crate::permuter::Permuter;

use lexical_sort::natural_lexical_cmp;
use sha2::Sha256;
use std::collections::HashMap;

// A Hex Encoded SHA2 Digest will always be 64 characters
// '<' + result.hash.len() + '>'
//  1  +       64          +  1
//                                = 66
const RESULT_HASH_CAPACITY: usize = 66;
const PERMUTATION_STRING_PATH_CAPACITY: usize = 128;

const NAME: &str = "URDNA2015";
const HASH_ALGORITHM: &str = "sha256";

type Hash = String;
type BlankNodeInfoMap<'a> = HashMap<String, BlankNodeInfo<'a>>;
type HashBlankNodeMap = HashMap<String, Vec<String>>;
type HashToRelatedMap = HashMap<String, Vec<String>>;

#[derive(Clone, Debug)]
struct HashNDegreeResult {
  hash: String,
  issuer: IdentifierIssuer,
}

#[derive(Clone, Debug, PartialEq)]
struct BlankNodeInfo<'a> {
  pub quads: Vec<&'a Quad>,
  hash: Option<Hash>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct URDNA2015<'a> {
  name: String,
  blank_node_info: BlankNodeInfoMap<'a>,
  canonical_issuer: IdentifierIssuer,
  hash_algorithm: String,
}

impl<'b> URDNA2015<'b> {
  pub fn new<'a>() -> URDNA2015<'a> {
    URDNA2015 {
      name: String::from(NAME),
      blank_node_info: BlankNodeInfoMap::new(),
      canonical_issuer: IdentifierIssuer::new("_:c14n"),
      hash_algorithm: String::from(HASH_ALGORITHM),
    }
  }

  // 4.4) Normalization Algorithm
  pub fn main(&mut self, dataset: &'b Dataset) -> String {
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
    let mut non_unique: Vec<&Vec<String>> = Vec::new();

    for hash in &hashes {
      // 5.4.1) If the length of identifier list is greater than 1,
      // continue to the next mapping.
      let id_list = hash_to_blank_nodes.get(hash).unwrap();
      if id_list.len() > 1 {
        non_unique.push(id_list);
        continue;
      }

      // 5.4.2) Use the Issue Identifier algorithm, passing canonical
      // issuer and the single blank node identifier in identifier
      // list, identifier, to issue a canonical replacement identifier
      // for identifier.
      let id = &id_list[0];
      self.canonical_issuer.get_id(id);

      // Note: These steps are skipped, optimized away since the loop
      // only needs to be run once.
      // 5.4.3) Remove identifier from non-normalized identifiers.
      // 5.4.4) Remove hash from the hash to blank nodes map.
      // 5.4.5) Set simple to true.
    }

    // 6) For each hash to identifier list mapping in hash to blank nodes map,
    // lexicographically-sorted by hash:
    // Note: sort optimized away, use `non_unique`.
    for id_list in non_unique {
      // 6.1) Create hash path list where each item will be a result of
      // running the Hash N-Degree Quads algorithm.
      let mut hash_path_list = Vec::with_capacity(id_list.len());
      // 6.2) For each blank node identifier identifier in identifier list:
      for id in id_list.iter() {
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
        issuer.get_id(id);

        // 6.2.4) Run the Hash N-Degree Quads algorithm, passing
        // temporary issuer, and append the result to the hash path list.
        let result = self.hash_n_degree_quads(&id, issuer);
        hash_path_list.push(result);
      }

      // 6.3) For each result in the hash path list,
      // lexicographically-sorted by the hash in result:
      hash_path_list.sort_by(|a, b| natural_lexical_cmp(&a.hash, &b.hash));
      for result in hash_path_list {
        // 6.3.1) For each blank node identifier, existing identifier,
        // that was issued a temporary identifier by identifier issuer
        // in result, issue a canonical identifier, in the same order,
        // using the Issue Identifier algorithm, passing canonical
        // issuer and existing identifier.
        let old_ids = result.issuer.get_old_ids();
        for id in old_ids {
          self.canonical_issuer.get_id(id);
        }
      }
    }

    /* Note: At this point all blank nodes in the set of RDF quads have been
    assigned canonical identifiers, which have been stored in the canonical
    issuer. Here each quad is updated by assigning each of its blank nodes
    its new identifier. */

    // 7) For each quad, quad, in input dataset:
    let mut normalized = Vec::with_capacity(quads.len());
    for quad in quads {
      // 7.1) Create a copy, quad copy, of quad and replace any existing
      // blank node identifiers using the canonical identifiers
      // previously issued by canonical issuer.

      // subject
      let s: nquads::Subject;
      let mut subject: Option<&nquads::Subject> = None;
      if Self::should_use_canonical_id(&quad.subject, &self.canonical_issuer) {
        s = nquads::Subject {
          term_type: quad.subject.term_type,
          value: self
            .canonical_issuer
            .get_existing_id(&quad.subject.value)
            .unwrap(),
        };
        subject = Some(&s);
      }

      // object
      let o: nquads::Object;
      let mut object: Option<&nquads::Object> = None;
      if Self::should_use_canonical_id(&quad.object, &self.canonical_issuer) {
        o = nquads::Object {
          term_type: quad.object.term_type,
          value: self
            .canonical_issuer
            .get_existing_id(&quad.object.value)
            .unwrap(),
          datatype: quad.object.get_datatype(),
          language: quad.object.get_language(),
        };
        object = Some(&o);
      }

      // graph
      let g: nquads::Graph;
      let mut graph: Option<&nquads::Graph> = None;
      if Self::should_use_canonical_id(&quad.graph, &self.canonical_issuer) {
        g = nquads::Graph {
          term_type: quad.graph.term_type,
          value: self
            .canonical_issuer
            .get_existing_id(&quad.graph.value)
            .unwrap(),
        };
        graph = Some(&g);
      }

      // 7.2) Add quad copy to the normalized dataset.
      if subject.is_none() && object.is_none() && graph.is_none() {
        // use existing quad when there is no need to create a clone
        normalized.push(nquads::serialize_quad(quad));
      } else {
        let quad_copy = nquads::QuadRef {
          subject: subject.or(Some(&quad.subject)).unwrap(),
          predicate: &quad.predicate,
          object: object.or(Some(&quad.object)).unwrap(),
          graph: graph.or(Some(&quad.graph)).unwrap(),
        };
        normalized.push(nquads::serialize_quad(&quad_copy));
      }
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
    // 2) Get the list of quads `quads` associated with the reference blank node
    // identifier in the blank node to quads map.
    // Note: We get the list of quads first and use its length to preallocate the
    // capacity of `serialized_quads` to prevent future reallocation.
    let mut info = self.blank_node_info.get_mut(id).unwrap();
    let mut serialized_quads: Vec<String> = Vec::with_capacity(info.quads.len());

    for quad in &mut info.quads {
      // 3.1) Serialize the quad in N-Quads format with the following special
      // rule:

      // 3.1.1) If any component in quad is an blank node, then serialize it
      // using a special identifier as follows:
      // 3.1.2) If the blank node's existing blank node identifier matches
      // the reference blank node identifier then use the blank node
      // identifier _:a, otherwise, use the blank node identifier _:z.

      // subject
      let s: nquads::Subject;
      let mut subject: Option<&nquads::Subject> = None;
      if quad.subject.term_type == TermType::BlankNode {
        s = nquads::Subject {
          term_type: TermType::BlankNode,
          value: if quad.subject.value == id {
            "_:a".to_string()
          } else {
            "_:z".to_string()
          },
        };
        subject = Some(&s);
      }

      // object
      let o: nquads::Object;
      let mut object: Option<&nquads::Object> = None;
      if quad.object.term_type == TermType::BlankNode {
        o = nquads::Object {
          term_type: TermType::BlankNode,
          value: if quad.object.value == id {
            "_:a".to_string()
          } else {
            "_:z".to_string()
          },
          datatype: quad.object.get_datatype(),
          language: quad.object.get_language(),
        };
        object = Some(&o);
      }

      // graph
      let g: nquads::Graph;
      let mut graph: Option<&nquads::Graph> = None;
      if quad.graph.term_type == TermType::BlankNode {
        g = nquads::Graph {
          term_type: TermType::BlankNode,
          value: if quad.graph.value == id {
            "_:a".to_string()
          } else {
            "_:z".to_string()
          },
        };
        graph = Some(&g);
      }

      if subject.is_none() && object.is_none() && graph.is_none() {
        // use existing quad when there is no need to create a clone
        serialized_quads.push(nquads::serialize_quad(*quad));
      } else {
        let quad_copy = nquads::QuadRef {
          subject: subject.or(Some(&quad.subject)).unwrap(),
          predicate: &quad.predicate,
          object: object.or(Some(&quad.object)).unwrap(),
          graph: graph.or(Some(&quad.graph)).unwrap(),
        };
        serialized_quads.push(nquads::serialize_quad(&quad_copy));
      }
    }

    // 4) Sort nquads in lexicographical order.
    serialized_quads.sort_unstable();

    // 5) Return the hash that results from passing the sorted, joined nquads
    // through the hash algorithm.
    let mut md: MessageDigest<Sha256> = MessageDigest::new();
    for quad in &serialized_quads {
      md.update(&quad);
    }
    let hex = MessageDigest::digest(md);
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
      id = self.canonical_issuer.get_id(related);
    } else if issuer.has_id(related) {
      id = issuer.get_id(related);
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
    MessageDigest::digest(md)
  }

  // 4.8) Hash N-Degree Quads
  fn hash_n_degree_quads(&mut self, id: &str, issuer: IdentifierIssuer) -> HashNDegreeResult {
    // 1) Create a hash to related blank nodes map for storing hashes that
    // identify related blank nodes.
    // Note: 2) and 3) handled within `create_hash_to_related`
    let mut md: MessageDigest<Sha256> = MessageDigest::new();
    let mut issuer = issuer;
    let mut hash_to_related = self.create_hash_to_related(id, &mut issuer);

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
      let mut blank_node_list = Vec::with_capacity(hash_to_related.len());
      for blank_node_id in hash_to_related.get_mut(&hash).unwrap() {
        blank_node_list.push(&blank_node_id[..]);
      }
      let mut elements = Permuter::permutation_elements(&mut blank_node_list);
      let mut element_refs = Vec::with_capacity(elements.len());
      for element in elements.iter_mut() {
        element_refs.push(element);
      }
      let permuter = Permuter::new(&mut element_refs);
      for permutation in permuter {
        // 5.4.1) Create a copy of issuer, issuer copy.
        let mut issuer_copy = issuer.clone();

        // 5.4.2) Create a string path.
        let mut path = String::with_capacity(PERMUTATION_STRING_PATH_CAPACITY);

        // 5.4.3) Create a recursion list, to store blank node identifiers
        // that must be recursively processed by this algorithm.
        let mut recursion_list: Vec<&str> = vec![];

        // 5.4.4) For each related in permutation:
        let mut next_permutation = false;
        for related in permutation.iter() {
          // 5.4.4.1) If a canonical identifier has been issued for
          // related, append it to path.
          if self.canonical_issuer.has_id(&related) {
            path.push_str(&self.canonical_issuer.get_id(related))
          } else {
            // 5.4.4.2) Otherwise:
            // 5.4.4.2.1) If issuer copy has not issued an identifier for
            // related, append related to recursion list.
            if !issuer_copy.has_id(&related) {
              recursion_list.push(related)
            }
            // 5.4.4.2.2) Use the Issue Identifier algorithm, passing
            // issuer copy and related and append the result to path.
            path.push_str(&issuer_copy.get_id(related))
          }

          // 5.4.4.3) If chosen path is not empty and the length of path
          // is greater than or equal to the length of chosen path and
          // path is lexicographically greater than chosen path, then
          // skip to the next permutation.
          // Note: Comparing path length to chosen path length can be optimized
          // away; only compare lexicographically.
          if chosen_path.is_empty() && path < chosen_path {
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
          let id = issuer_copy.get_id(related);
          let result = self.hash_n_degree_quads(related, issuer_copy);
          // copy and related and append the result to path.
          path.push_str(&id);

          // 5.4.5.3) Append <, the hash in result, and > to path.
          let mut result_hash = String::with_capacity(RESULT_HASH_CAPACITY);
          result_hash.push('<');
          result_hash.push_str(&result.hash);
          result_hash.push('>');
          path.push_str(&result_hash);

          // 5.4.5.4) Set issuer copy to the identifier issuer in
          // result.
          issuer_copy = result.issuer;

          // 5.4.5.5) If chosen path is not empty and the length of path
          // is greater than or equal to the length of chosen path and
          // path is lexicographically greater than chosen path, then
          // skip to the next permutation.
          // Note: Comparing path length to chosen path length can be optimized
          // away; only compare lexicographically.
          if chosen_path.is_empty() && path < chosen_path {
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
        if chosen_path.is_empty() || path < chosen_path {
          chosen_path = path;
          chosen_issuer = issuer_copy;
        }
      }

      // 5.5) Append chosen path to data to hash.
      md.update(&chosen_path);

      // 5.6) Replace issuer, by reference, with chosen issuer.
      issuer = chosen_issuer;
    }

    HashNDegreeResult {
      hash: MessageDigest::digest(md),
      issuer,
    }
  }

  // helper for getting a related predicate
  fn get_related_predicate(&self, quad: &Quad) -> String {
    let mut related_predicate = String::with_capacity(nquads::DEFAULT_TERM_CAPACITY);

    // append "<quad.predicate.value>"
    related_predicate.push('<');
    related_predicate.push_str(quad.predicate.get_value());
    related_predicate.push('>');

    related_predicate
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
    let quads = self.blank_node_info.get(id).unwrap().quads.clone();

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

  fn add_blank_node_quad_info<'a, T>(&'a mut self, quad: &'b Quad, component: &T)
  where
    T: Term,
  {
    if *component.get_term_type() != TermType::BlankNode {
      return;
    }

    let id = component.get_value();
    if let Some(info) = self.blank_node_info.get_mut(id) {
      info.quads.push(quad);
    } else {
      let quads = vec![quad];
      self
        .blank_node_info
        .insert(id.to_string(), BlankNodeInfo { quads, hash: None });
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
    if !(*component.get_term_type() == TermType::BlankNode && related != id) {
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
      entries.push(related.to_string());
    } else {
      hash_to_related.insert(hash, vec![related.to_string()]);
    }
  }

  fn should_use_canonical_id<T>(copy: &T, issuer: &IdentifierIssuer) -> bool
  where
    T: Term,
  {
    if *copy.get_term_type() == TermType::BlankNode && !copy.get_value().starts_with(&issuer.prefix)
    {
      return true;
    }

    false
  }
}

fn hashmap_keys_to_vec<T: Clone, U>(hashmap: &HashMap<T, U>) -> Vec<T> {
  hashmap.keys().cloned().collect()
}
