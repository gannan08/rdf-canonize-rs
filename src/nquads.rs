extern crate regex;

use regex::Regex;
use std::collections::HashMap;

// define partial regexes
const IRI: &str = "(?:<([^:]+:[^>]*)>)";
const PLAIN: &str = "\"([^\"\\\\]*(?:\\\\.[^\"\\\\]*)*)\"";
const LANGUAGE: &str = "(?:@([a-zA-Z]+(?:-[a-zA-Z0-9]+)*))";
const WS: &str = "[ \\t]+";
const WSO: &str = "[ \\t]*";

// XSD constants
const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";

// RDF constants
const RDF_LANGSTRING: &str =
  "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString";

#[derive(Clone, Debug, PartialEq)]
pub enum TermType {
  BlankNode,
  NamedNode,
  Literal,
  DefaultGraph,
  None,
}

pub trait Term {
  fn new() -> Self;
  fn get_term_type(&self) -> TermType;
  fn set_term_type(&mut self, term_type: &TermType);
  fn get_value(&self) -> String;
  fn set_value(&mut self, value: &str);
}

#[derive(Clone, Debug, PartialEq)]
pub struct Subject {
  term_type: TermType,
  value: String,
}

impl Term for Subject {
  fn new() -> Subject {
    Subject {
      term_type: TermType::None,
      value: String::from(""),
    }
  }

  fn get_term_type(&self) -> TermType {
    self.term_type.clone()
  }

  fn set_term_type(&mut self, term_type: &TermType) {
    self.term_type = term_type.clone();
  }

  fn get_value(&self) -> String {
    self.value.clone()
  }

  fn set_value(&mut self, value: &str) {
    self.value = value.to_string();
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Predicate {
  term_type: TermType,
  value: String,
}

impl Term for Predicate {
  fn new() -> Predicate {
    Predicate {
      term_type: TermType::None,
      value: String::from(""),
    }
  }

  fn get_term_type(&self) -> TermType {
    self.term_type.clone()
  }

  fn set_term_type(&mut self, term_type: &TermType) {
    self.term_type = term_type.clone();
  }

  fn get_value(&self) -> String {
    self.value.clone()
  }

  fn set_value(&mut self, value: &str) {
    self.value = value.to_string();
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
  term_type: TermType,
  value: String,
  datatype: Option<String>,
  language: Option<String>,
}

impl Term for Object {
  fn new() -> Object {
    Object {
      term_type: TermType::None,
      value: String::from(""),
      datatype: None,
      language: None,
    }
  }

  fn get_term_type(&self) -> TermType {
    self.term_type.clone()
  }

  fn set_term_type(&mut self, term_type: &TermType) {
    self.term_type = term_type.clone();
  }

  fn get_value(&self) -> String {
    self.value.clone()
  }

  fn set_value(&mut self, value: &str) {
    self.value = value.to_string();
  }
}

impl Object {
  pub fn get_language(&self) -> Option<String> {
    self.language.clone()
  }

  pub fn set_language(&mut self, language: &str) {
    self.language = Some(language.to_string());
  }

  pub fn get_datatype(&self) -> Option<String> {
    self.datatype.clone()
  }

  pub fn set_datatype(&mut self, datatype: &str) {
    self.datatype = Some(datatype.to_string());
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Graph {
  term_type: TermType,
  value: String,
}

impl Term for Graph {
  fn new() -> Graph {
    Graph {
      term_type: TermType::None,
      value: String::from(""),
    }
  }

  fn get_term_type(&self) -> TermType {
    self.term_type.clone()
  }

  fn set_term_type(&mut self, term_type: &TermType) {
    self.term_type = term_type.clone();
  }

  fn get_value(&self) -> String {
    self.value.clone()
  }

  fn set_value(&mut self, value: &str) {
    self.value = value.to_string();
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Quad {
  pub subject: Subject,
  pub predicate: Predicate,
  pub object: Object,
  pub graph: Graph,
}

impl Quad {
  pub fn new() -> Quad {
    Quad {
      subject: Subject::new(),
      predicate: Predicate::new(),
      object: Object::new(),
      graph: Graph::new(),
    }
  }
}

pub type QuadSet = Vec<Quad>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Dataset {
  pub quads: QuadSet,
  graph_map: HashMap<String, Vec<usize>>,
}

impl Dataset {
  pub fn new() -> Dataset {
    Dataset {
      quads: Vec::new(),
      graph_map: HashMap::new(),
    }
  }

  pub fn add(&mut self, quad: Quad) -> bool {
    let graph = quad.graph.clone();
    let graph_name = graph.value;
    match self.graph_map.get_mut(&graph_name) {
      Some(quad_ptrs) => {
        quad_ptrs.push(self.quads.len());
      }
      None => {
        let mut quad_ptrs = Vec::new();
        quad_ptrs.push(self.quads.len());
        self.graph_map.insert(graph_name, quad_ptrs);
      }
    }

    self.quads.push(quad);

    true
  }
}

pub fn serialize_quad(quad: &Quad) -> String {
  let s = &quad.subject;
  let p = &quad.predicate;
  let o = &quad.object;
  let g = &quad.graph;

  let mut nquad = Vec::<String>::new();

  // subject can only be NamedNode or BlankNode
  if s.term_type == TermType::NamedNode {
    nquad.push(format!("<{}>", s.value));
  } else {
    nquad.push(s.value.to_string());
  }

  // predicate can only be NamedNode
  nquad.push(format!(" <{}> ", p.value));

  // object is NamedNode, BlankNode, or Literal
  if o.term_type == TermType::NamedNode {
    nquad.push(format!("<{}>", o.value));
  } else if o.term_type == TermType::BlankNode {
    nquad.push(o.value.to_string())
  } else {
    nquad.push(format!("\"{}\"", escape_string(&o.value)));
    if let Some(datatype) = &o.datatype {
      if datatype == RDF_LANGSTRING {
        match &o.language {
          Some(language) => nquad.push(format!("@{}", language)),
          None => {}
        }
      } else if datatype != XSD_STRING {
        nquad.push(format!("^^<{}>", datatype))
      }
    }
  }

  // graph can only be NamedNode or BlankNode (or DefaultGraph, but that
  // does not add to `nquad`)
  if g.term_type == TermType::NamedNode {
    nquad.push(format!(" <{}>", g.value));
  } else if g.term_type == TermType::BlankNode {
    nquad.push(format!(" {}", g.value));
  }

  nquad.push(String::from(" .\n"));
  nquad.join("")
}
pub fn parse_nquads(dataset: &str) -> Dataset {
  let lines = dataset.lines();

  let mut rdf_dataset = Dataset::new();

  for line in lines {
    println!("{}", line);
    let quad = parse_nquad(&line);
    rdf_dataset.add(quad);
  }

  rdf_dataset
}

lazy_static! {
  // https://www.w3.org/TR/turtle/#grammar-production-BLANK_NODE_LABEL
  static ref PN_CHARS_BASE: String = format!(
    "{}{}{}{}{}{}{}{}{}{}{}{}{}",
    "A-Z",
    "a-z",
    "\u{00C0}-\u{00D6}",
    "\u{00D8}-\u{00F6}",
    "\u{00F8}-\u{02FF}",
    "\u{0370}-\u{037D}",
    "\u{037F}-\u{1FFF}",
    "\u{200C}-\u{200D}",
    "\u{2070}-\u{218F}",
    "\u{2C00}-\u{2FEF}",
    "\u{3001}-\u{D7FF}",
    "\u{F900}-\u{FDCF}",
    "\u{FDF0}-\u{FFFD}"
    // TODO:
    // "\u{1000}0-\u{EFFF}F"
  );
  static ref PN_CHARS_U: String = format!(
    "{}{}",
    PN_CHARS_BASE.as_str(),
    "_"
  );
  static ref PN_CHARS: String = format!(
    "{}{}{}{}{}{}",
    PN_CHARS_U.as_str(),
    "0-9",
    "-",
    "\u{00B7}",
    "\u{0300}-\u{036F}",
    "\u{203F}-\u{2040}"
  );
  // define partial regexes
  static ref BLANK_NODE_LABEL: String = format!(
    "{}{}{}{}{}{}{}{}{}{}",
    "(_:",
      "(?:[", PN_CHARS_U.as_str(), "0-9])",
      "(?:(?:[" , PN_CHARS.as_str() , ".])*(?:[" , PN_CHARS.as_str() , "]))?",
    ")"
  );
  static ref BNODE: String = BLANK_NODE_LABEL.clone();
  static ref DATATYPE: String = format!("{}{}{}", "(?:\\^\\^", IRI, ")");
  static ref LITERAL: String = format!("(?:{}(?:{}|{})?)", PLAIN, DATATYPE.as_str(), LANGUAGE);

  // define quad part regexes
  static ref SUBJECT: String = format!("(?:{}|{}){}", IRI, BNODE.as_str(), WS);
  static ref PROPERTY: String = format!("{}{}", IRI, WS);
  static ref OBJECT: String = format!("(?:{}|{}|{}){}", IRI, BNODE.as_str(), LITERAL.as_str(), WSO);
  static ref GRAPH: String = format!("(?:\\.|(?:(?:{}|{}){}\\.))", IRI, BNODE.as_str(), WSO);

  // full quad regex
  static ref QUAD: String = format!(
      "^{}{}{}{}{}{}$",
      WSO,
      SUBJECT.as_str(),
      PROPERTY.as_str(),
      OBJECT.as_str(),
      GRAPH.as_str(),
      WSO
  );


  static ref QUAD_REGEX: Regex = Regex::new(&QUAD).unwrap();
}

pub fn parse_nquad(serialized_triple: &str) -> Quad {
  let group = QUAD_REGEX.captures(serialized_triple).unwrap();
  //  the capture group indexed at 1. This is because the entire match is
  //  stored in the capture group at index 0
  let subject = parse_subject(&group);
  let predicate = parse_predicate(&group);
  let object = parse_object(&group);
  let graph_name = parse_graph_name(&group);

  Quad {
    subject: subject.unwrap(),
    predicate: predicate.unwrap(),
    object: object.unwrap(),
    graph: graph_name.unwrap(),
  }
}

fn parse_subject(group: &regex::Captures) -> Option<Subject> {
  let subject = match group.get(1) {
    Some(val) => Some(Subject {
      term_type: TermType::NamedNode,
      value: String::from(val.as_str()),
    }),
    None => Some(Subject {
      term_type: TermType::BlankNode,
      value: String::from(group.get(2).unwrap().as_str()),
    }),
  };

  Some(subject.unwrap())
}

fn parse_predicate(group: &regex::Captures) -> Option<Predicate> {
  let predicate = match group.get(3) {
    Some(val) => Some(Predicate {
      term_type: TermType::NamedNode,
      value: String::from(val.as_str()),
    }),
    None => None,
  };

  Some(predicate.unwrap())
}

fn parse_object(group: &regex::Captures) -> Option<Object> {
  if let Some(value) = group.get(4) {
    let object = Object {
      term_type: TermType::NamedNode,
      value: String::from(value.as_str()),
      datatype: None,
      language: None,
    };
    return Some(object);
  } else if let Some(value) = group.get(5) {
    let object = Object {
      term_type: TermType::BlankNode,
      value: String::from(value.as_str()),
      datatype: None,
      language: None,
    };
    return Some(object);
  }

  let escaped = String::from(group.get(6).unwrap().as_str());
  let unescaped = unescape_string(&escaped);

  if let Some(datatype) = group.get(7) {
    let object = Object {
      term_type: TermType::Literal,
      value: unescaped,
      datatype: Some(String::from(datatype.as_str())),
      language: None,
    };
    return Some(object);
  } else if let Some(language) = group.get(8) {
    let object = Object {
      term_type: TermType::Literal,
      value: unescaped,
      datatype: Some(String::from(RDF_LANGSTRING)),
      language: Some(String::from(language.as_str())),
    };
    return Some(object);
  }

  let object = Object {
    term_type: TermType::Literal,
    value: unescaped,
    datatype: Some(String::from(XSD_STRING)),
    language: None,
  };
  Some(object)
}

fn parse_graph_name(group: &regex::Captures) -> Option<Graph> {
  if let Some(value) = group.get(9) {
    let graph_name = String::from(value.as_str());
    let graph = Graph {
      term_type: TermType::NamedNode,
      value: graph_name,
    };
    return Some(graph);
  } else if let Some(value) = group.get(10) {
    let graph_name = String::from(value.as_str());
    let graph = Graph {
      term_type: TermType::BlankNode,
      value: graph_name,
    };
    return Some(graph);
  }

  let graph = Graph {
    term_type: TermType::DefaultGraph,
    value: String::from("@default"),
  };
  Some(graph)
}

fn escape_string(unescaped: &str) -> String {
  let mut escaped = unescaped.to_string();

  escaped = escaped.replace("\\", "\\\\");
  escaped = escaped.replace("\r", "\\r");
  escaped = escaped.replace("\n", "\\n");
  escaped = escaped.replace("\"", "\\\"");

  escaped
}

fn unescape_string(escaped: &str) -> String {
  let mut unescaped = escaped.to_string();

  unescaped = unescaped.replace("\\t", "\t");
  // Must use hex for escape sequence
  // see: https://github.com/rust-lang/rfcs/issues/751
  unescaped = unescaped.replace("\\b", "\x08");
  unescaped = unescaped.replace("\\n", "\n");
  unescaped = unescaped.replace("\\r", "\r");
  // Must use hex for escape sequence
  // see: https://github.com/rust-lang/rfcs/issues/751
  unescaped = unescaped.replace("\\f", "\x0C");
  unescaped = unescaped.replace("\\\"", "\"");
  unescaped = unescaped.replace("\'", "'");
  unescaped = unescaped.replace("\\\\", "\\");

  unescaped
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn subject_equals() {
    let subject_a = Subject {
      term_type: TermType::NamedNode,
      value: String::from("foobar"),
    };
    let subject_b = Subject {
      term_type: TermType::NamedNode,
      value: String::from("foobar"),
    };
    assert_eq!(subject_a, subject_b);
  }

  #[test]
  fn subject_not_equals() {
    let subject_a = Subject {
      term_type: TermType::NamedNode,
      value: String::from("ganesh"),
    };
    let subject_b = Subject {
      term_type: TermType::BlankNode,
      value: String::from("ganesh"),
    };
    assert_ne!(subject_a, subject_b);
  }

  #[test]
  fn predicate_equals() {
    let predicate_a = Predicate {
      term_type: TermType::NamedNode,
      value: String::from("ganesh"),
    };
    let predicate_b = Predicate {
      term_type: TermType::NamedNode,
      value: String::from("ganesh"),
    };
    assert_eq!(predicate_a, predicate_b);
  }

  #[test]
  fn predicate_not_equals() {
    let predicate_a = Predicate {
      term_type: TermType::NamedNode,
      value: String::from("ganesh"),
    };
    let predicate_b = Predicate {
      term_type: TermType::NamedNode,
      value: String::from("annan"),
    };
    assert_ne!(predicate_a, predicate_b);
  }

  #[test]
  fn object_equals() {
    let object_a = Object {
      term_type: TermType::NamedNode,
      value: String::from("ganesh"),
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    let object_b = Object {
      term_type: TermType::NamedNode,
      value: String::from("ganesh"),
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    assert_eq!(object_a, object_b);
  }

  #[test]
  fn object_not_equals() {
    let object_a = Object {
      term_type: TermType::NamedNode,
      value: String::from("ganesh"),
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    let object_b = Object {
      term_type: TermType::NamedNode,
      value: String::from("ganesh"),
      datatype: Some(String::from("http://example.com/t2")),
      language: Some(String::from("fr")),
    };
    assert_ne!(object_a, object_b);
  }

  #[test]
  fn graph_equals() {
    let graph_a = Graph {
      term_type: TermType::NamedNode,
      value: String::from("@default"),
    };
    let graph_b = Graph {
      term_type: TermType::NamedNode,
      value: String::from("@default"),
    };
    assert_eq!(graph_a, graph_b);
  }

  #[test]
  fn graph_not_equals() {
    let graph_a = Graph {
      term_type: TermType::NamedNode,
      value: String::from("@default"),
    };
    let graph_b = Graph {
      term_type: TermType::NamedNode,
      value: String::from("_:b10"),
    };
    assert_ne!(graph_a, graph_b);
  }

  #[test]
  fn quad_equals() {
    let subject = Subject {
      term_type: TermType::NamedNode,
      value: String::from("foobar"),
    };
    let predicate = Predicate {
      term_type: TermType::NamedNode,
      value: String::from("ganesh"),
    };
    let object = Object {
      term_type: TermType::NamedNode,
      value: String::from("ganesh"),
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    let graph = Graph {
      term_type: TermType::NamedNode,
      value: String::from("@default"),
    };

    let quad_a = Quad {
      subject: subject.clone(),
      predicate: predicate.clone(),
      object: object.clone(),
      graph: graph.clone(),
    };
    let quad_b = Quad {
      subject,
      predicate,
      object,
      graph,
    };
    assert_eq!(quad_a, quad_b);
  }

  #[test]
  fn quad_not_equals() {
    let subject = Subject {
      term_type: TermType::NamedNode,
      value: String::from("foobar"),
    };
    let predicate = Predicate {
      term_type: TermType::NamedNode,
      value: String::from("ganesh"),
    };
    let object = Object {
      term_type: TermType::NamedNode,
      value: String::from("ganesh"),
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    let graph_a = Graph {
      term_type: TermType::NamedNode,
      value: String::from("@default"),
    };
    let graph_b = Graph {
      term_type: TermType::NamedNode,
      value: String::from("_:b10"),
    };

    let quad_a = Quad {
      subject: subject.clone(),
      predicate: predicate.clone(),
      object: object.clone(),
      graph: graph_a,
    };
    let quad_b = Quad {
      subject,
      predicate,
      object,
      graph: graph_b,
    };
    assert_ne!(quad_a, quad_b);
  }
}
