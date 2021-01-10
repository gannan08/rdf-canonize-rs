extern crate regex;

use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;

use hyperscan::chimera::prelude::*;

// use rio_api::parser::QuadsParser;
// use rio_turtle::{NQuadsParser, TurtleError};
// use rio_api::model::{NamedOrBlankNode};

// define default capacities
pub const DEFAULT_NQUAD_CAPACITY: usize = 256;
pub const DEFAULT_TERM_CAPACITY: usize = 64;

// define partial regexes
const IRI: &str = "(?:<([^:]+:[^>]*)>)";
const PLAIN: &str = "\"([^\"\\\\]*(?:\\\\.[^\"\\\\]*)*)\"";
const LANGUAGE: &str = "(?:@([a-zA-Z]+(?:-[a-zA-Z0-9]+)*))";
const WS: &str = "[ \\t]+";
const WSO: &str = "[ \\t]*";

// XSD constants
const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";

// RDF constants
const RDF_LANGSTRING: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString";

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TermType {
  BlankNode,
  NamedNode,
  Literal,
  DefaultGraph,
  None,
}

pub trait Term<'a> {
  fn new() -> Self;
  fn get_term_type(&self) -> &TermType;
  fn set_term_type(&mut self, term_type: &TermType);
  fn get_value(&self) -> &str;
  fn set_value(&mut self, value: &'a str);
}

#[derive(Clone, Debug, PartialEq)]
pub struct Subject<'a> {
  pub term_type: TermType,
  pub value: &'a str,
}

impl<'a> Term<'a> for Subject<'a> {
  fn new() -> Subject<'a> {
    Subject {
      term_type: TermType::None,
      value: "",
    }
  }

  fn get_term_type(&self) -> &TermType {
    &self.term_type
  }

  fn set_term_type(&mut self, term_type: &TermType) {
    self.term_type = *term_type;
  }

  fn get_value(&self) -> &str {
    &self.value
  }

  fn set_value(&mut self, value: &'a str) {
    self.value = value;
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Predicate<'a> {
  pub term_type: TermType,
  pub value: &'a str,
}

impl<'a> Term<'a> for Predicate<'a> {
  fn new() -> Predicate<'a> {
    Predicate {
      term_type: TermType::None,
      value: "",
    }
  }

  fn get_term_type(&self) -> &TermType {
    &self.term_type
  }

  fn set_term_type(&mut self, term_type: &TermType) {
    self.term_type = *term_type;
  }

  fn get_value(&self) -> &str {
    &self.value
  }

  fn set_value(&mut self, value: &'a str) {
    self.value = value;
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object<'a> {
  pub term_type: TermType,
  pub value: &'a str,
  pub datatype: Option<String>,
  pub language: Option<String>,
}

impl<'a> Term<'a> for Object<'a> {
  fn new() -> Object<'a> {
    Object {
      term_type: TermType::None,
      value: "",
      datatype: None,
      language: None,
    }
  }

  fn get_term_type(&self) -> &TermType {
    &self.term_type
  }

  fn set_term_type(&mut self, term_type: &TermType) {
    self.term_type = *term_type;
  }

  fn get_value(&self) -> &str {
    &self.value
  }

  fn set_value(&mut self, value: &'a str) {
    self.value = value;
  }
}

impl Object<'_> {
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
pub struct Graph<'a> {
  pub term_type: TermType,
  pub value: &'a str,
}

impl<'a> Term<'a> for Graph<'a> {
  fn new() -> Graph<'a> {
    Graph {
      term_type: TermType::None,
      value: "",
    }
  }

  fn get_term_type(&self) -> &TermType {
    &self.term_type
  }

  fn set_term_type(&mut self, term_type: &TermType) {
    self.term_type = *term_type;
  }

  fn get_value(&self) -> &str {
    &self.value
  }

  fn set_value(&mut self, value: &'a str) {
    self.value = value;
  }
}

pub trait QuadSerialize<'a> {
  fn get_subject(&'a self) -> &'a Subject;
  fn get_predicate(&'a self) -> &'a Predicate;
  fn get_object(&'a self) -> &'a Object;
  fn get_graph(&'a self) -> &'a Graph;
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuadRef<'a> {
  pub subject: &'a Subject<'a>,
  pub predicate: &'a Predicate<'a>,
  pub object: &'a Object<'a>,
  pub graph: &'a Graph<'a>,
}

impl QuadSerialize<'_> for QuadRef<'_> {
  fn get_subject(&self) -> &Subject {
    self.subject
  }

  fn get_predicate(&self) -> &Predicate {
    self.predicate
  }

  fn get_object(&self) -> &Object {
    self.object
  }

  fn get_graph(&self) -> &Graph {
    self.graph
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Quad<'a> {
  pub subject: Subject<'a>,
  pub predicate: Predicate<'a>,
  pub object: Object<'a>,
  pub graph: Graph<'a>,
}

impl<'a> Quad<'a> {
  pub fn new() -> Quad<'a> {
    Self::default()
  }
}

impl QuadSerialize<'_> for Quad<'_> {
  fn get_subject(&self) -> &Subject {
    &self.subject
  }

  fn get_predicate(&self) -> &Predicate {
    &self.predicate
  }

  fn get_object(&self) -> &Object {
    &self.object
  }

  fn get_graph(&self) -> &Graph {
    &self.graph
  }
}

impl Default for Quad<'_> {
  fn default() -> Quad<'static> {
    Quad {
      subject: Subject::new(),
      predicate: Predicate::new(),
      object: Object::new(),
      graph: Graph::new(),
    }
  }
}

pub type QuadSet<'a> = Vec<Quad<'a>>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Dataset<'a> {
  pub quads: QuadSet<'a>,
  graph_map: HashMap<String, Vec<usize>>,
}

impl<'a> Dataset<'a> {
  pub fn new() -> Dataset<'a> {
    Dataset {
      quads: Vec::new(),
      graph_map: HashMap::new(),
    }
  }

  pub fn add(&mut self, quad: Quad<'a>) -> bool {
    let graph = quad.graph.clone();
    let graph_name = graph.value;
    match self.graph_map.get_mut(graph_name) {
      Some(quad_ptrs) => {
        quad_ptrs.push(self.quads.len());
      }
      None => {
        let mut quad_ptrs = Vec::new();
        quad_ptrs.push(self.quads.len());
        self.graph_map.insert(graph_name.to_string(), quad_ptrs);
      }
    }

    self.quads.push(quad);

    true
  }
}

pub fn serialize_quad<'a, T>(quad: &'a T) -> String
where
  T: QuadSerialize<'a>,
{
  let s = quad.get_subject();
  let p = quad.get_predicate();
  let o = quad.get_object();
  let g = quad.get_graph();

  let mut nquad = String::with_capacity(DEFAULT_NQUAD_CAPACITY);

  // subject can only be NamedNode or BlankNode
  if s.term_type == TermType::NamedNode {
    // append "<subject.value>"
    nquad.push('<');
    nquad.push_str(&s.value);
    nquad.push('>');
  } else {
    // append "subject.value"
    nquad.push_str(&s.value);
  }

  // predicate can only be NamedNode
  // append " <predicate.value> "
  nquad.push(' ');
  nquad.push('<');
  nquad.push_str(&p.value);
  nquad.push('>');
  nquad.push(' ');

  // object is NamedNode, BlankNode, or Literal
  if o.term_type == TermType::NamedNode {
    // append "<object.value>"
    nquad.push('<');
    nquad.push_str(&o.value);
    nquad.push('>');
  } else if o.term_type == TermType::BlankNode {
    // append "object.value"
    nquad.push_str(&o.value)
  } else {
    // append "\"escape(object.value)\""
    nquad.push('\"');
    nquad.push_str(&escape_string(o.value));
    nquad.push('\"');
    if let Some(datatype) = &o.datatype {
      if datatype == RDF_LANGSTRING {
        if let Some(language) = &o.language {
          // append "@language"
          nquad.push('@');
          nquad.push_str(&language);
        }
      } else if datatype != XSD_STRING {
        // append "^^<datatype>"
        nquad.push('^');
        nquad.push('^');
        nquad.push('<');
        nquad.push_str(&datatype);
        nquad.push('>');
      }
    }
  }

  // graph can only be NamedNode or BlankNode (or DefaultGraph, but that
  // does not add to `nquad`)
  if g.term_type == TermType::NamedNode {
    // append " <graph.value>"
    nquad.push(' ');
    nquad.push('<');
    nquad.push_str(&g.value);
    nquad.push('>');
  } else if g.term_type == TermType::BlankNode {
    // append " graph.value"
    nquad.push(' ');
    nquad.push_str(&g.value);
  }

  // append " .\n"
  nquad.push(' ');
  nquad.push('.');
  nquad.push('\n');
  nquad
}

// pub fn parse_nquads_ol(dataset: &str) -> Dataset {
//   let lines = dataset.lines();

//   let mut rdf_dataset = Dataset::new();

//   for line in lines {
//     let quad = parse_nquad(&line);
//     println!("{:?}", quad.predicate);
//     rdf_dataset.add(quad);
//   }

//   rdf_dataset
// }

pub fn parse_nquads(dataset: &str) -> Dataset {
  let mut rdf_dataset = Dataset::new();

  let mut errors = vec![];
  let hyper_scratch: Scratch = HYPER_DB.alloc_scratch().unwrap();

  for line in dataset.lines() {
      HYPER_DB.scan(
        line,
        &hyper_scratch,
        |_id, _from, _to, _flags, captured: Option<&[Capture]>| {
            if let Some(captured) = captured {
                let subject = match captured[1].is_active() {
                  true => Subject {
                    term_type: TermType::NamedNode,
                    value: &line[captured[1].range()],
                  },
                  false => Subject {
                    term_type: TermType::BlankNode,
                    value: &line[captured[2].range()],
                  },
                };
                // println!("SSSSSSSS {:?}", subject);
                let predicate = Predicate {
                  term_type: TermType::NamedNode,
                  value: &line[captured[3].range()],
                };
                // println!("PPPPPPPP {:?}", predicate);
                // println!("PPPPPP {:?}", line);
                // println!("00000 {:?}", captured[0].range());
                // println!("11111 {:?}", captured[1].range());
                // println!("22222 {:?}", captured[2].range());
                // println!("33333 {:?}", captured[3].range());
                // println!("44444 {:?}", captured[4].range());
                // println!("44444 {:?}", captured);
                // println!("55555 {:?}", captured[4].range());
                // println!("66666 {:?}", captured[4].range());
                // println!("77777 {:?}", captured[4].range());

                let object;
                if captured[4].is_active() {
                  object = Object {
                    term_type: TermType::NamedNode,
                    value: &line[captured[4].range()],
                    datatype: None,
                    language: None,
                  };
                } else if captured[5].is_active() {
                  object = Object {
                    term_type: TermType::BlankNode,
                    value: &line[captured[5].range()],
                    datatype: None,
                    language: None,
                  };
                } else {
                  // FIXME: how to do this!?
                  // let escaped = String::from(group.get(6).unwrap().as_str());
                  // let unescaped = unescape_string(&escaped);
                  let should_be_unescaped = &line[captured[6].range()];
                  if captured.len() >= 8 {
                    if captured[7].is_active() {
                      object = Object {
                        term_type: TermType::Literal,
                        value: &should_be_unescaped,
                        datatype: Some(String::from(&line[captured[7].range()])),
                        language: None,
                      };
                    } else {
                      object = Object {
                        term_type: TermType::Literal,
                        value: &should_be_unescaped,
                        datatype: Some(String::from(RDF_LANGSTRING)),
                        language: Some(String::from(&line[captured[8].range()])),
                      };
                    }
                  } else {
                    object = Object {
                      term_type: TermType::Literal,
                      value: &should_be_unescaped,
                      datatype: Some(String::from(XSD_STRING)),
                      language: None,
                    }
                  }
                }
                // println!("OOOOOOO {:?}", object);
                let graph;
                if captured.len() >= 10 {
                  if captured[9].is_active() {
                    graph = Graph {
                      term_type: TermType::NamedNode,
                      value: &line[captured[9].range()],
                    };
                  } else {
                    graph = Graph {
                      term_type: TermType::BlankNode,
                      value: &line[captured[10].range()],
                    };
                  }
                } else {
                  graph = Graph {
                    term_type: TermType::DefaultGraph,
                    value: "@default",
                  }
                }
                // println!("GGGG {:?}", graph);
                rdf_dataset.add(Quad {
                  subject,
                  predicate,
                  object,
                  graph,
                });
            }
            Matching::Continue
        },
        |error_type, id| {
            errors.push((error_type, id));
            Matching::Skip
        },
    )
    .unwrap();

    // for group in QUAD_REGEX.captures(line) {
    //   let subject = match group.get(1) {
    //     Some(value) => Subject {
    //       term_type: TermType::NamedNode,
    //       value: value.as_str(),
    //     },
    //     None => Subject {
    //       term_type: TermType::BlankNode,
    //       value: group.get(2).unwrap().as_str(),
    //     }
    //   };
    //   let predicate = Predicate {
    //     term_type: TermType::NamedNode,
    //     value: group.get(3).unwrap().as_str(),
    //   };

    //   let object;
    //   if let Some(value) = group.get(4) {
    //     object = Object {
    //       term_type: TermType::NamedNode,
    //       value: value.as_str(),
    //       datatype: None,
    //       language: None,
    //     };
    //   } else if let Some(value) = group.get(5) {
    //     object = Object {
    //       term_type: TermType::BlankNode,
    //       value: value.as_str(),
    //       datatype: None,
    //       language: None,
    //     };
    //   } else {
    //     // FIXME: how to do this!?
    //     // let escaped = String::from(group.get(6).unwrap().as_str());
    //     // let unescaped = unescape_string(&escaped);
    //     let should_be_unescacped = group.get(6).unwrap().as_str();

    //     if let Some(datatype) = group.get(7) {
    //       object = Object {
    //         term_type: TermType::Literal,
    //         value: &should_be_unescacped,
    //         datatype: Some(String::from(datatype.as_str())),
    //         language: None,
    //       };
    //     } else if let Some(language) = group.get(8) {
    //       object = Object {
    //         term_type: TermType::Literal,
    //         value: &should_be_unescacped,
    //         datatype: Some(String::from(RDF_LANGSTRING)),
    //         language: Some(String::from(language.as_str())),
    //       };
    //     } else {
    //       object = Object {
    //         term_type: TermType::Literal,
    //         value: &should_be_unescacped,
    //         datatype: Some(String::from(XSD_STRING)),
    //         language: None,
    //       }
    //     }
    //   }

    //   let graph;
    //   if let Some(value) = group.get(9) {
    //     graph = Graph {
    //       term_type: TermType::NamedNode,
    //       value: value.as_str(),
    //     };
    //   } else if let Some(value) = group.get(10) {
    //     graph = Graph {
    //       term_type: TermType::BlankNode,
    //       value: value.as_str(),
    //     };
    //   } else {
    //     graph = Graph {
    //       term_type: TermType::DefaultGraph,
    //       value: "@default",
    //     }
    //   }

    //   rdf_dataset.add(Quad {
    //     subject,
    //     predicate,
    //     object,
    //     graph,
    //   });
    // };
  }

  rdf_dataset
}

// pub fn parse_nquads_5<'a>(dataset: &'a str, groups: &'a mut Vec<&regex::Captures<'a>>) -> Dataset<'a> {
//   let lines = dataset.lines();

//   let mut rdf_dataset = Dataset::new();

//   for line in lines.into_iter() {
//     let idx = match QUAD_REGEX.captures(line) {
//       Some(group) => {
//         groups.push(group);
//         groups.len() - 1
//       },
//       None => panic!()
//     };
//     let group = groups[idx];
//     let quad = parse_nquad(&group);
//     rdf_dataset.add(quad);
//   }

//   rdf_dataset
// }

#[allow(unused_must_use)]
// pub fn parse_nquads_rio(dataset: &str) -> Dataset {
//   let mut rdf_dataset = Dataset::new();

//   NQuadsParser::new(dataset.as_ref()).parse_all(& mut |t| {
//     let id = match t.subject {
//       NamedOrBlankNode::NamedNode(node) => node.iri,
//       NamedOrBlankNode::BlankNode(node) => node.id,
//     };
//     let subject = Subject {
//       term_type: term_type(&rio_api::model::Term::from(t.subject)),
//       value: id,
//     };
//     let predicate = Predicate {
//       term_type: TermType::NamedNode,
//       value: t.predicate.iri.to_string(),
//     };
//     let object = object_data(&rio_api::model::Term::from(t.object));

//     let graph;
//     if let Some(graph_name) = t.graph_name {
//       graph = graph_data(&rio_api::model::Term::from(graph_name));
//     } else {
//       graph = Graph {
//         term_type: TermType::DefaultGraph,
//         value: String::from(""),
//       };
//     }
//     let quad = Quad {
//       subject,
//       predicate,
//       object,
//       graph,
//     };
//     rdf_dataset.add(quad);

//     Ok(()) as Result<(), TurtleError>
//   });

//   rdf_dataset
// }

// fn graph_data(rio_term: &rio_api::model::Term) -> Graph {
//   match rio_term {
//     rio_api::model::Term::NamedNode(node) => Graph {
//       term_type: TermType::NamedNode,
//       value: node.iri.to_string(),
//     },
//     rio_api::model::Term::BlankNode(node) => Graph {
//       term_type: TermType::BlankNode,
//       value: node.to_string(),
//     },
//     _ => panic!(),
//   }
// }

// fn object_data(rio_term: &rio_api::model::Term) -> Object {
//   match rio_term {
//     rio_api::model::Term::Literal(literal) => match literal {
//       rio_api::model::Literal::Simple { value } => Object {
//         term_type: TermType::Literal,
//         value: value.to_string(),
//         language: None,
//         datatype: Some(XSD_STRING.to_string()),
//       },
//       rio_api::model::Literal::LanguageTaggedString { value, language } => Object {
//         term_type: TermType::Literal,
//         value: value.to_string(),
//         language: Some(language.to_string()),
//         datatype: Some(RDF_LANGSTRING.to_string()),
//       },
//       rio_api::model::Literal::Typed { value, datatype } => Object {
//         term_type: TermType::Literal,
//         value: value.to_string(),
//         language: None,
//         datatype: Some(datatype.to_string()),
//       },
//     },
//     rio_api::model::Term::BlankNode(node) => Object {
//       term_type: TermType::BlankNode,
//       value: node.to_string(),
//       language: None,
//       datatype: None,
//     },
//     rio_api::model::Term::NamedNode(node) => Object {
//       term_type: TermType::NamedNode,
//       value: node.iri.to_string(),
//       language: None,
//       datatype: None,
//     },
//   }
// }

// fn term_type(rio_term: &rio_api::model::Term) -> TermType {
//   match rio_term {
//     rio_api::model::Term::NamedNode(_) => TermType::NamedNode,
//     rio_api::model::Term::BlankNode(_) => TermType::BlankNode,
//     rio_api::model::Term::Literal(_) => TermType::Literal,
//   }
// }

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

  static ref QUAD_HYPER_PATTERN: Pattern = QUAD.parse().unwrap();

  static ref HYPER_DB: Database = QUAD_HYPER_PATTERN.with_groups().unwrap();
}

// pub fn parse_nquad<'a>(serialized_triple: &'a str) -> Quad<'a> {
// pub fn parse_nquad<'a>(group: &'a regex::Captures<'a>) -> Quad<'a> {
//   let subject = parse_subject(&group);
//   let predicate = parse_predicate(&group);
//   let object = parse_object(&group);
//   let graph = parse_graph(&group);

//   Quad {
//     subject,
//     predicate,
//     object,
//     graph,
//   }
// }

// fn parse_subject<'a>(group: &'a regex::Captures) -> Subject<'a> {
//   let subject = match group.get(1) {
//     Some(value) => Subject {
//       term_type: TermType::NamedNode,
//       value: value.as_str(),
//     },
//     None => Subject {
//       term_type: TermType::BlankNode,
//       value: group.get(2).unwrap().as_str(),
//     },
//   };

//   subject
// }

// fn parse_predicate<'a>(group: &'a regex::Captures) -> Predicate<'a> {
//   let value = group.get(3).unwrap();

//   Predicate {
//     term_type: TermType::NamedNode,
//     value: value.as_str(),
//   }
// }

// fn parse_object<'a>(group: &'a regex::Captures) -> Object<'a> {
//   if let Some(value) = group.get(4) {
//     let object = Object {
//       term_type: TermType::NamedNode,
//       value: value.as_str(),
//       datatype: None,
//       language: None,
//     };
//     return object;
//   } else if let Some(value) = group.get(5) {
//     let object = Object {
//       term_type: TermType::BlankNode,
//       value: value.as_str(),
//       datatype: None,
//       language: None,
//     };
//     return object;
//   }

//   let escaped = String::from(group.get(6).unwrap().as_str());
//   let unescaped = unescape_string(&escaped);

//   if let Some(datatype) = group.get(7) {
//     let object = Object {
//       term_type: TermType::Literal,
//       value: &unescaped,
//       datatype: Some(String::from(datatype.as_str())),
//       language: None,
//     };
//     return object;
//   } else if let Some(language) = group.get(8) {
//     let object = Object {
//       term_type: TermType::Literal,
//       value: &unescaped,
//       datatype: Some(String::from(RDF_LANGSTRING)),
//       language: Some(String::from(language.as_str())),
//     };
//     return object;
//   }

//   Object {
//     term_type: TermType::Literal,
//     value: &unescaped,
//     datatype: Some(String::from(XSD_STRING)),
//     language: None,
//   }
// }

// fn parse_graph<'a>(group: &'a regex::Captures) -> Graph<'a> {
//   if let Some(value) = group.get(9) {
//     let graph = Graph {
//       term_type: TermType::NamedNode,
//       value: value.as_str(),
//     };
//     return graph;
//   } else if let Some(value) = group.get(10) {
//     let graph = Graph {
//       term_type: TermType::BlankNode,
//       value: value.as_str(),
//     };
//     return graph;
//   }

//   Graph {
//     term_type: TermType::DefaultGraph,
//     value: "@default",
//   }
// }

fn escape_string<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
  lazy_static! {
    static ref REGEX: Regex = Regex::new("[\\\\\n\r\"]").unwrap();
  }
  let input = input.into();
  let first = REGEX.find(&input);
  if let Some(first) = first {
    // create a vector from the beginning of the string up to and not including the first occurence
    // of a character that needs to be escaped
    let mut output: Vec<u8> = Vec::from(input[0..first.start()].as_bytes());
    // Max capacity for an escape string will be N * ESCAPE_CHARS_LEN, where N is the largest factor
    // a single character can grow. We assert that N is 2 since we are only adding the '\'
    // character. Therefore, the max capacity of the escaped string is 2 * ESCAPE_CHARS_LEN.
    // TLDR: create a large enough buffer to prevent reallocations
    output.reserve((input.len() - first.start()) * 2);
    // iterate over remaining characters and escape
    let rest = input[first.start()..].bytes();
    for c in rest {
      match c {
        b'\\' => output.extend_from_slice(b"\\\\"),
        b'\r' => output.extend_from_slice(b"\\r"),
        b'\n' => output.extend_from_slice(b"\\n"),
        b'\"' => output.extend_from_slice(b"\\\""),
        _ => output.push(c),
      }
    }

    return Cow::Owned(String::from_utf8(output).unwrap());
  }

  input
}

// fn unescape_string(escaped: &str) -> String {
//   let mut unescaped = escaped.to_string();

//   unescaped = unescaped.replace("\\t", "\t");
//   // Must use hex for escape sequence
//   // see: https://github.com/rust-lang/rfcs/issues/751
//   unescaped = unescaped.replace("\\b", "\x08");
//   unescaped = unescaped.replace("\\n", "\n");
//   unescaped = unescaped.replace("\\r", "\r");
//   // Must use hex for escape sequence
//   // see: https://github.com/rust-lang/rfcs/issues/751
//   unescaped = unescaped.replace("\\f", "\x0C");
//   unescaped = unescaped.replace("\\\"", "\"");
//   unescaped = unescaped.replace("\'", "'");
//   unescaped = unescaped.replace("\\\\", "\\");

//   unescaped
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn subject_equals() {
    let subject_a = Subject {
      term_type: TermType::NamedNode,
      value: "foobar",
    };
    let subject_b = Subject {
      term_type: TermType::NamedNode,
      value: "foobar",
    };
    assert_eq!(subject_a, subject_b);
  }

  #[test]
  fn subject_not_equals() {
    let subject_a = Subject {
      term_type: TermType::NamedNode,
      value: "ganesh",
    };
    let subject_b = Subject {
      term_type: TermType::BlankNode,
      value: "ganesh",
    };
    assert_ne!(subject_a, subject_b);
  }

  #[test]
  fn predicate_equals() {
    let predicate_a = Predicate {
      term_type: TermType::NamedNode,
      value: "ganesh",
    };
    let predicate_b = Predicate {
      term_type: TermType::NamedNode,
      value: "ganesh",
    };
    assert_eq!(predicate_a, predicate_b);
  }

  #[test]
  fn predicate_not_equals() {
    let predicate_a = Predicate {
      term_type: TermType::NamedNode,
      value: "ganesh",
    };
    let predicate_b = Predicate {
      term_type: TermType::NamedNode,
      value: "annan",
    };
    assert_ne!(predicate_a, predicate_b);
  }

  #[test]
  fn object_equals() {
    let object_a = Object {
      term_type: TermType::NamedNode,
      value: "ganesh",
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    let object_b = Object {
      term_type: TermType::NamedNode,
      value: "ganesh",
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    assert_eq!(object_a, object_b);
  }

  #[test]
  fn object_not_equals() {
    let object_a = Object {
      term_type: TermType::NamedNode,
      value: "ganesh",
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    let object_b = Object {
      term_type: TermType::NamedNode,
      value: "ganesh",
      datatype: Some(String::from("http://example.com/t2")),
      language: Some(String::from("fr")),
    };
    assert_ne!(object_a, object_b);
  }

  #[test]
  fn graph_equals() {
    let graph_a = Graph {
      term_type: TermType::NamedNode,
      value: "@default",
    };
    let graph_b = Graph {
      term_type: TermType::NamedNode,
      value: "@default",
    };
    assert_eq!(graph_a, graph_b);
  }

  #[test]
  fn graph_not_equals() {
    let graph_a = Graph {
      term_type: TermType::NamedNode,
      value: "@default",
    };
    let graph_b = Graph {
      term_type: TermType::NamedNode,
      value: "_:b10",
    };
    assert_ne!(graph_a, graph_b);
  }

  #[test]
  fn quad_equals() {
    let subject = Subject {
      term_type: TermType::NamedNode,
      value: "foobar",
    };
    let predicate = Predicate {
      term_type: TermType::NamedNode,
      value: "ganesh",
    };
    let object = Object {
      term_type: TermType::NamedNode,
      value: "ganesh",
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    let graph = Graph {
      term_type: TermType::NamedNode,
      value: "@default",
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
      value: "foobar",
    };
    let predicate = Predicate {
      term_type: TermType::NamedNode,
      value: "ganesh",
    };
    let object = Object {
      term_type: TermType::NamedNode,
      value: "ganesh",
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    let graph_a = Graph {
      term_type: TermType::NamedNode,
      value: "@default",
    };
    let graph_b = Graph {
      term_type: TermType::NamedNode,
      value: "_:b10",
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
