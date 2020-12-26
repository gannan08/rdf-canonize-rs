#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test044_canonize() {
        // normalize/tests/test044-in.nq
        let dataset_str = r#"_:b0 <http://example.org/vocab#p> _:b1 .
_:b0 <http://example.org/vocab#p> _:b2 .
_:b0 <http://example.org/vocab#p> _:b3 .
_:b1 <http://example.org/vocab#p> _:b0 .
_:b1 <http://example.org/vocab#p> _:b3 .
_:b1 <http://example.org/vocab#p> _:b4 .
_:b2 <http://example.org/vocab#p> _:b0 .
_:b2 <http://example.org/vocab#p> _:b4 .
_:b2 <http://example.org/vocab#p> _:b5 .
_:b3 <http://example.org/vocab#p> _:b0 .
_:b3 <http://example.org/vocab#p> _:b1 .
_:b3 <http://example.org/vocab#p> _:b5 .
_:b4 <http://example.org/vocab#p> _:b1 .
_:b4 <http://example.org/vocab#p> _:b2 .
_:b4 <http://example.org/vocab#p> _:b5 .
_:b5 <http://example.org/vocab#p> _:b3 .
_:b5 <http://example.org/vocab#p> _:b2 .
_:b5 <http://example.org/vocab#p> _:b4 .
_:b6 <http://example.org/vocab#p> _:b7 .
_:b6 <http://example.org/vocab#p> _:b8 .
_:b6 <http://example.org/vocab#p> _:b9 .
_:b7 <http://example.org/vocab#p> _:b6 .
_:b7 <http://example.org/vocab#p> _:b10 .
_:b7 <http://example.org/vocab#p> _:b11 .
_:b8 <http://example.org/vocab#p> _:b6 .
_:b8 <http://example.org/vocab#p> _:b10 .
_:b8 <http://example.org/vocab#p> _:b11 .
_:b9 <http://example.org/vocab#p> _:b6 .
_:b9 <http://example.org/vocab#p> _:b10 .
_:b9 <http://example.org/vocab#p> _:b11 .
_:b10 <http://example.org/vocab#p> _:b7 .
_:b10 <http://example.org/vocab#p> _:b8 .
_:b10 <http://example.org/vocab#p> _:b9 .
_:b11 <http://example.org/vocab#p> _:b7 .
_:b11 <http://example.org/vocab#p> _:b8 .
_:b11 <http://example.org/vocab#p> _:b9 .
"#;

        let rdf_dataset = rdf_canonize::nquads::parse_nquads(&dataset_str);
        let f = rdf_canonize::canonize(&rdf_dataset, "URDNA2015");
        println!("FFFF {}", f.unwrap());
    }

    #[bench]
    fn bench009_canonize(b: &mut Bencher) {
        // normalization/tests/test009-urdna2015.nq
        let dataset_str = r#"<http://example.org/test#chapter> <http://purl.org/dc/elements/1.1/description> "Fun" .
<http://example.org/test#chapter> <http://purl.org/dc/elements/1.1/title> "Chapter One" .
<http://example.org/test#jane> <http://example.org/vocab#authored> <http://example.org/test#chapter> .
<http://example.org/test#jane> <http://xmlns.com/foaf/0.1/name> "Jane" .
<http://example.org/test#john> <http://xmlns.com/foaf/0.1/name> "John" .
<http://example.org/test#library> <http://example.org/vocab#contains> <http://example.org/test#book> .
<http://example.org/test#book> <http://example.org/vocab#contains> <http://example.org/test#chapter> .
<http://example.org/test#book> <http://purl.org/dc/elements/1.1/contributor> "Writer" .
<http://example.org/test#book> <http://purl.org/dc/elements/1.1/title> "My Book" .
"#;

        let rdf_dataset = rdf_canonize::nquads::parse_nquads(&dataset_str);

        b.iter(|| rdf_canonize::canonize(&rdf_dataset, "URDNA2015").unwrap())
    }

    #[bench]
    fn bench044_canonize(b: &mut Bencher) {
        // normalize/tests/test044-urdna2015.nq
        let dataset_str = r#"_:b0 <http://example.org/vocab#p> _:b1 .
_:b0 <http://example.org/vocab#p> _:b2 .
_:b0 <http://example.org/vocab#p> _:b3 .
_:b1 <http://example.org/vocab#p> _:b0 .
_:b1 <http://example.org/vocab#p> _:b3 .
_:b1 <http://example.org/vocab#p> _:b4 .
_:b2 <http://example.org/vocab#p> _:b0 .
_:b2 <http://example.org/vocab#p> _:b4 .
_:b2 <http://example.org/vocab#p> _:b5 .
_:b3 <http://example.org/vocab#p> _:b0 .
_:b3 <http://example.org/vocab#p> _:b1 .
_:b3 <http://example.org/vocab#p> _:b5 .
_:b4 <http://example.org/vocab#p> _:b1 .
_:b4 <http://example.org/vocab#p> _:b2 .
_:b4 <http://example.org/vocab#p> _:b5 .
_:b5 <http://example.org/vocab#p> _:b3 .
_:b5 <http://example.org/vocab#p> _:b2 .
_:b5 <http://example.org/vocab#p> _:b4 .
_:b6 <http://example.org/vocab#p> _:b7 .
_:b6 <http://example.org/vocab#p> _:b8 .
_:b6 <http://example.org/vocab#p> _:b9 .
_:b7 <http://example.org/vocab#p> _:b6 .
_:b7 <http://example.org/vocab#p> _:b10 .
_:b7 <http://example.org/vocab#p> _:b11 .
_:b8 <http://example.org/vocab#p> _:b6 .
_:b8 <http://example.org/vocab#p> _:b10 .
_:b8 <http://example.org/vocab#p> _:b11 .
_:b9 <http://example.org/vocab#p> _:b6 .
_:b9 <http://example.org/vocab#p> _:b10 .
_:b9 <http://example.org/vocab#p> _:b11 .
_:b10 <http://example.org/vocab#p> _:b7 .
_:b10 <http://example.org/vocab#p> _:b8 .
_:b10 <http://example.org/vocab#p> _:b9 .
_:b11 <http://example.org/vocab#p> _:b7 .
_:b11 <http://example.org/vocab#p> _:b8 .
_:b11 <http://example.org/vocab#p> _:b9 .
"#;

        let rdf_dataset = rdf_canonize::nquads::parse_nquads(&dataset_str);

        b.iter(|| rdf_canonize::canonize(&rdf_dataset, "URDNA2015").unwrap())
    }

    #[bench]
    fn bench044_parse(b: &mut Bencher) {
        // normalize/tests/test044-in.nq
        let dataset_str = r#"_:b0 <http://example.org/vocab#p> _:b1 .
_:b0 <http://example.org/vocab#p> _:b2 .
_:b0 <http://example.org/vocab#p> _:b3 .
_:b1 <http://example.org/vocab#p> _:b0 .
_:b1 <http://example.org/vocab#p> _:b3 .
_:b1 <http://example.org/vocab#p> _:b4 .
_:b2 <http://example.org/vocab#p> _:b0 .
_:b2 <http://example.org/vocab#p> _:b4 .
_:b2 <http://example.org/vocab#p> _:b5 .
_:b3 <http://example.org/vocab#p> _:b0 .
_:b3 <http://example.org/vocab#p> _:b1 .
_:b3 <http://example.org/vocab#p> _:b5 .
_:b4 <http://example.org/vocab#p> _:b1 .
_:b4 <http://example.org/vocab#p> _:b2 .
_:b4 <http://example.org/vocab#p> _:b5 .
_:b5 <http://example.org/vocab#p> _:b3 .
_:b5 <http://example.org/vocab#p> _:b2 .
_:b5 <http://example.org/vocab#p> _:b4 .
_:b6 <http://example.org/vocab#p> _:b7 .
_:b6 <http://example.org/vocab#p> _:b8 .
_:b6 <http://example.org/vocab#p> _:b9 .
_:b7 <http://example.org/vocab#p> _:b6 .
_:b7 <http://example.org/vocab#p> _:b10 .
_:b7 <http://example.org/vocab#p> _:b11 .
_:b8 <http://example.org/vocab#p> _:b6 .
_:b8 <http://example.org/vocab#p> _:b10 .
_:b8 <http://example.org/vocab#p> _:b11 .
_:b9 <http://example.org/vocab#p> _:b6 .
_:b9 <http://example.org/vocab#p> _:b10 .
_:b9 <http://example.org/vocab#p> _:b11 .
_:b10 <http://example.org/vocab#p> _:b7 .
_:b10 <http://example.org/vocab#p> _:b8 .
_:b10 <http://example.org/vocab#p> _:b9 .
_:b11 <http://example.org/vocab#p> _:b7 .
_:b11 <http://example.org/vocab#p> _:b8 .
_:b11 <http://example.org/vocab#p> _:b9 .
"#;

        b.iter(|| rdf_canonize::nquads::parse_nquads(&dataset_str))
    }
}
