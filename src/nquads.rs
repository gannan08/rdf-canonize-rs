extern crate regex;

use regex::Regex;

// define partial regexes
const IRI: &str = "(?:<([^:]+:[^>]*)>)";
const BNODE: &str = "(_:(?:[A-Za-z][A-Za-z0-9]*))";
const PLAIN: &str = r#""([^"\\]*(?:\\.[^"\\]*)*)""#;
const LANGUAGE: &str = "(?:@([a-zA-Z]+(?:-[a-zA-Z0-9]+)*))";
const WS: &str = "[ \\t]+";
const WSO: &str = "[ \\t]*";

// XSD constants
const XSD_BOOLEAN: &str = "http://www.w3.org/2001/XMLSchema#boolean";
const XSD_DOUBLE: &str = "http://www.w3.org/2001/XMLSchema#double";
const XSD_INTEGER: &str = "http://www.w3.org/2001/XMLSchema#integer";
const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";

// RDF constants
const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
const RDF_LANGSTRING: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString";

lazy_static! {
    // define partial regexes
    static ref DATATYPE: String = format!("{}{}{}", r#"(?:\^\^"#, IRI, ")");
    static ref LITERAL: String = format!("(?:{}(?:{}|{})?)", PLAIN, DATATYPE.as_str(), LANGUAGE);
    static ref EMPTY: String = format!("{}{}$", r#"^"#, WSO);

    // define quad part regexes
    static ref SUBJECT: String = format!("(?:{}|{}){}", IRI, BNODE, WS);
    static ref PROPERTY: String = format!("{}{}", IRI, WS);
    static ref OBJECT: String = format!("(?:{}|{}|{}){}", IRI, BNODE, LITERAL.as_str(), WSO);
    static ref GRAPH: String = format!("(?:\\.|(?:(?:{}|{}){}\\.))", IRI, BNODE, WSO);

    // full quad regex
    static ref QUAD: String = format!(
        "{}{}{}{}{}{}{}$",
        r#"^"#,
        WSO,
        SUBJECT.as_str(),
        PROPERTY.as_str(),
        OBJECT.as_str(),
        GRAPH.as_str(),
        WSO
    );
    static ref QUAD_REGEX: Regex = Regex::new(&QUAD).unwrap();
}

#[derive(Debug, PartialEq)]
enum TermType {
  BlankNode,
  IRI,
  NamedNode,
  Literal,
  DefaultGraph,
}

#[derive(Debug, PartialEq)]
struct Subject {
  term_type: TermType,
  value: String,
}

#[derive(Debug, PartialEq)]
struct Predicate {
  term_type: TermType,
  value: String,
}

#[derive(Debug, PartialEq)]
struct Object {
  term_type: TermType,
  value: String,
  datatype: Option<String>,
  language: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Quad {
  subject: Subject,
  predicate: Predicate,
  object: Object,
  graph: String,
}

#[derive(Debug, PartialEq)]
pub struct Dataset {
  pub quads: Vec<Quad>,
}

impl Dataset {
  fn new() -> Dataset {
    Dataset { quads: Vec::new() }
  }
}

pub fn parse_nquads(dataset: &str) -> Dataset {
  let lines = dataset.lines();

  let mut rdf_dataset = Dataset::new();

  for line in lines {
    println!("{}", line);
    let quad = parse_nquad(&line);
    rdf_dataset.quads.push(quad);
  }

  rdf_dataset
}

pub fn parse_nquad(serialized_triple: &str) -> Quad {
  let group = QUAD_REGEX.captures(serialized_triple).unwrap();
  //  the capture group indexed at 1. This is because the entire match is
  //  stored in the capture group at index 0
  for i in 1..group.len() {
    match group.get(i) {
      Some(val) => println!("{}: {}", i, val.as_str()),
      None => println!("{}: None", i),
    }
  }

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
      term_type: TermType::IRI,
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
      term_type: TermType::IRI,
      value: String::from(val.as_str()),
    }),
    None => None,
  };

  Some(predicate.unwrap())
}

fn parse_object(group: &regex::Captures) -> Option<Object> {
  if let Some(value) = group.get(4) {
    return Some(Object {
      term_type: TermType::IRI,
      value: String::from(value.as_str()),
      datatype: None,
      language: None,
    });
  } else if let Some(value) = group.get(5) {
    return Some(Object {
      term_type: TermType::BlankNode,
      value: String::from(value.as_str()),
      datatype: None,
      language: None,
    });
  }

  let unescaped = String::from(group.get(6).unwrap().as_str());
  let escaped = escape_string(&unescaped);

  if let Some(datatype) = group.get(7) {
    return Some(Object {
      term_type: TermType::Literal,
      value: escaped,
      datatype: Some(String::from(datatype.as_str())),
      language: None,
    });
  } else if let Some(language) = group.get(8) {
    return Some(Object {
      term_type: TermType::Literal,
      value: escaped,
      datatype: Some(String::from(RDF_LANGSTRING)),
      language: Some(String::from(language.as_str())),
    });
  }

  Some(Object {
    term_type: TermType::Literal,
    value: escaped,
    datatype: Some(String::from(XSD_STRING)),
    language: None,
  })
}

fn parse_graph_name(group: &regex::Captures) -> Option<String> {
  let name = match group.get(9) {
    Some(val) => Some(val.as_str()),
    None => match group.get(10) {
      Some(val) => Some(val.as_str()),
      None => Some("@default"),
    },
  };

  Some(String::from(name.unwrap()))
}

fn escape_string(unescaped: &str) -> String {
  let mut escaped;

  escaped = unescaped.replace(r#"\\""#, r#"""#);
  escaped = escaped.replace(r#"\\t"#, "\t");
  escaped = escaped.replace(r#"\\n"#, "\n");
  escaped = escaped.replace(r#"\\r"#, "\r");
  escaped = escaped.replace(r#"\\\\""#, r#"\\"#);

  escaped
}

#[cfg(test)]
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn subject_equals() {
    let subject_a = Subject {
      term_type: TermType::IRI,
      value: String::from("foobar"),
    };
    let subject_b = Subject {
      term_type: TermType::IRI,
      value: String::from("foobar"),
    };
    assert_eq!(subject_a, subject_b);
  }

  #[test]
  fn subject_not_equals() {
    let subject_a = Subject {
      term_type: TermType::IRI,
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
      term_type: TermType::IRI,
      value: String::from("ganesh"),
    };
    let predicate_b = Predicate {
      term_type: TermType::IRI,
      value: String::from("ganesh"),
    };
    assert_eq!(predicate_a, predicate_b);
  }

  #[test]
  fn predicate_not_equals() {
    let predicate_a = Predicate {
      term_type: TermType::IRI,
      value: String::from("ganesh"),
    };
    let predicate_b = Predicate {
      term_type: TermType::IRI,
      value: String::from("annan"),
    };
    assert_ne!(predicate_a, predicate_b);
  }

  #[test]
  fn object_equals() {
    let object_a = Object {
      term_type: TermType::IRI,
      value: String::from("ganesh"),
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    let object_b = Object {
      term_type: TermType::IRI,
      value: String::from("ganesh"),
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    assert_eq!(object_a, object_b);
  }

  #[test]
  fn object_not_equals() {
    let object_a = Object {
      term_type: TermType::IRI,
      value: String::from("ganesh"),
      datatype: Some(String::from("http://example.com/t2")),
      language: None,
    };
    let object_b = Object {
      term_type: TermType::IRI,
      value: String::from("ganesh"),
      datatype: Some(String::from("http://example.com/t2")),
      language: Some(String::from("fr")),
    };
    assert_ne!(object_a, object_b);
  }
}
