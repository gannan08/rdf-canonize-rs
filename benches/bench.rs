#![feature(test)]

extern crate test;

const VERES_ONE_DID: &str = r#"_:b0 <http://purl.org/dc/terms/creator> <https://ashburn.capybara.veres.one/consensus/continuity2017/voters/z6MkgTBtCodgNvf1SaQLRbCppkVMo7BggAP4NohtPY8ZNqic> .
_:b0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/webledger#CreateWebLedgerRecord> .
_:b0 <https://w3id.org/security#proof> _:b1 .
_:b0 <https://w3id.org/security#proof> _:b3 .
_:b0 <https://w3id.org/webledger#record> "{\"@context\":[\"https://w3id.org/did/v0.11\",\"https://w3id.org/veres-one/v1\"],\"assertionMethod\":[{\"controller\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d\",\"id\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d#z6MkqVQKNUG994U8mK6p7CX6PMtijsgDuhQBUEXgfAPCqQEP\",\"publicKeyBase58\":\"C39GnE1hoWyfepG7RdZFYGLivJQNVp9pnDckptRBvBT1\",\"type\":\"Ed25519VerificationKey2018\"}],\"authentication\":[{\"controller\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d\",\"id\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d#z6MkjXWUNtoT9e1BDSt5zLmMuY7k9u99euxCQiduXCTwrY2u\",\"publicKeyBase58\":\"65FRneZ1p6Wi6x3PJmoX4SZkLKsJF2hqihiygvVvwKFX\",\"type\":\"Ed25519VerificationKey2018\"}],\"capabilityDelegation\":[{\"controller\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d\",\"id\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d#z6MknXoqGqfAG7vjBTPqNZjf1HWcMD2c5csTCkdkY62S8BRy\",\"publicKeyBase58\":\"95YngbQivaSG4xZ8gzmpABxcXdkkfjd6Wjiphp4RCxeb\",\"type\":\"Ed25519VerificationKey2018\"}],\"capabilityInvocation\":[{\"controller\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d\",\"id\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d#z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d\",\"publicKeyBase58\":\"3bKogxzDN4y6wv8rVncv23YEB3YW6MCWZUCVEAoGufMF\",\"type\":\"Ed25519VerificationKey2018\"}],\"id\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d\"}"^^<http://www.w3.org/1999/02/22-rdf-syntax-ns#JSON> .
_:b2 <http://purl.org/dc/terms/created> "2021-01-09T20:50:08Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> _:b1 .
_:b2 <http://purl.org/dc/terms/creator> <did:v1:test:nym:z279yHL6HsxRzCPU78DAWgZVieb8xPK1mJKJBbP8T2CezuFY#z279yHL6HsxRzCPU78DAWgZVieb8xPK1mJKJBbP8T2CezuFY> _:b1 .
_:b2 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2018> _:b1 .
_:b2 <https://w3id.org/security#capability> <did:v1:uuid:c37e914a-1e2a-4d59-9668-ee93458fd19a> _:b1 .
_:b2 <https://w3id.org/security#capabilityAction> "write" _:b1 .
_:b2 <https://w3id.org/security#jws> "MOCKPROOF" _:b1 .
_:b2 <https://w3id.org/security#proofPurpose> <https://w3id.org/security#capabilityInvocationMethod> _:b1 .
_:b4 <http://purl.org/dc/terms/created> "2021-01-09T20:50:08Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> _:b3 .
_:b4 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2018> _:b3 .
_:b4 <https://w3id.org/security#capability> <did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d> _:b3 .
_:b4 <https://w3id.org/security#capabilityAction> "create" _:b3 .
_:b4 <https://w3id.org/security#jws> "eyJhbGciOiJFZERTQSIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..u_T7y7P_woiPmpxfnY0rDdA_o25A9m9BOUfXu4zc1PqfIs92Po8sJn_D2xSPI2Ijuz22T6YibLtud1NgvFO1BQ" _:b3 .
_:b4 <https://w3id.org/security#proofPurpose> <https://w3id.org/security#capabilityInvocationMethod> _:b3 .
_:b4 <https://w3id.org/security#verificationMethod> <did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d#z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d> _:b3 .
"#;

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

    #[test]
    fn test_merge_canonize() {
        // merge event
        let dataset_str = r#"_:b0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/webledger#ContinuityMergeEvent> .
_:b0 <https://w3id.org/security#proof> _:b1 .
_:b0 <https://w3id.org/webledger#parentHash> "zQmPkZrQs9dyezAQkVniqkMjm5nP3cdWFBzNsnnFLrsNf9u" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmYDcw6hXTZHCYaPyuGLCo8jcNREidQs4ikwKdVyS5uwKA" .
_:b0 <https://w3id.org/webledger#parentHash> "zQma45eMXmzKBXYwLdU7FvAEW3ekMy4fJjqEQVhYQFgwYAP" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmb6eicGxT6FAAZdxEzam2JpPu8ajiMJYhzPnhgHJJKh8f" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmc6b7weYQEu2NBDK9DB4HBc4bt2qQGbkvkEZBW6ajJ5F7" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmdxvSCwPjTvx3SAN2XHZ4uQpHKpbnHmns9BF8uZASW6Lx" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmePs3zy2fLPEsBXqGn2LPWSGYbzPy7CZTTz1f2ng3ysph" .
_:b0 <https://w3id.org/webledger#treeHash> "zQmPkZrQs9dyezAQkVniqkMjm5nP3cdWFBzNsnnFLrsNf9u" .
_:b2 <http://purl.org/dc/terms/created> "2018-12-21T23:40:20Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> _:b1 .
_:b2 <http://purl.org/dc/terms/creator> <https://bedrock.localhost:18443/consensus/continuity2017/voters/z6MkkabTusFkLnquxwHwCm28v59UX3P9Pn5scvc7fCaNvWUL> _:b1 .
_:b2 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2018> _:b1 .
_:b2 <https://w3id.org/security#jws> "eyJhbGciOiJFZERTQSIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..JJ5c7mF7ru9XhPtrNqj1s6J74yqOC0HcNyK_Wa0OcfDaiODZFIJ2dXIrc_qqqvTWynIqJid6yXkKsGAzyi_HDQ" _:b1 .
"#;

        let rdf_dataset = rdf_canonize::nquads::parse_nquads(&dataset_str);
        let f = rdf_canonize::canonize(&rdf_dataset, "URDNA2015");
        println!("FFFF {}", f.unwrap());
    }

    #[test]
    fn test_veres_one_did_canonize() {
        let rdf_dataset = rdf_canonize::nquads::parse_nquads(VERES_ONE_DID);
        let f = rdf_canonize::canonize(&rdf_dataset, "URDNA2015");
        println!("{}", f.unwrap());
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
    fn bench_merge_canonize(b: &mut Bencher) {
        // normalization/tests/test009-urdna2015.nq
        let dataset_str = r#"_:b0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/webledger#ContinuityMergeEvent> .
_:b0 <https://w3id.org/security#proof> _:b1 .
_:b0 <https://w3id.org/webledger#parentHash> "zQmPkZrQs9dyezAQkVniqkMjm5nP3cdWFBzNsnnFLrsNf9u" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmYDcw6hXTZHCYaPyuGLCo8jcNREidQs4ikwKdVyS5uwKA" .
_:b0 <https://w3id.org/webledger#parentHash> "zQma45eMXmzKBXYwLdU7FvAEW3ekMy4fJjqEQVhYQFgwYAP" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmb6eicGxT6FAAZdxEzam2JpPu8ajiMJYhzPnhgHJJKh8f" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmc6b7weYQEu2NBDK9DB4HBc4bt2qQGbkvkEZBW6ajJ5F7" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmdxvSCwPjTvx3SAN2XHZ4uQpHKpbnHmns9BF8uZASW6Lx" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmePs3zy2fLPEsBXqGn2LPWSGYbzPy7CZTTz1f2ng3ysph" .
_:b0 <https://w3id.org/webledger#treeHash> "zQmPkZrQs9dyezAQkVniqkMjm5nP3cdWFBzNsnnFLrsNf9u" .
_:b2 <http://purl.org/dc/terms/created> "2018-12-21T23:40:20Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> _:b1 .
_:b2 <http://purl.org/dc/terms/creator> <https://bedrock.localhost:18443/consensus/continuity2017/voters/z6MkkabTusFkLnquxwHwCm28v59UX3P9Pn5scvc7fCaNvWUL> _:b1 .
_:b2 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2018> _:b1 .
_:b2 <https://w3id.org/security#jws> "eyJhbGciOiJFZERTQSIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..JJ5c7mF7ru9XhPtrNqj1s6J74yqOC0HcNyK_Wa0OcfDaiODZFIJ2dXIrc_qqqvTWynIqJid6yXkKsGAzyi_HDQ" _:b1 .
"#;

        let rdf_dataset = rdf_canonize::nquads::parse_nquads(&dataset_str);

        b.iter(|| rdf_canonize::canonize(&rdf_dataset, "URDNA2015").unwrap())
    }

    #[bench]
    fn bench_veres_one_did_canonize(b: &mut Bencher) {
        let rdf_dataset = rdf_canonize::nquads::parse_nquads(VERES_ONE_DID);
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

    #[bench]
    fn bench_merge_parse(b: &mut Bencher) {
        let dataset_str = r#"_:b0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/webledger#ContinuityMergeEvent> .
_:b0 <https://w3id.org/security#proof> _:b1 .
_:b0 <https://w3id.org/webledger#parentHash> "zQmPkZrQs9dyezAQkVniqkMjm5nP3cdWFBzNsnnFLrsNf9u" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmYDcw6hXTZHCYaPyuGLCo8jcNREidQs4ikwKdVyS5uwKA" .
_:b0 <https://w3id.org/webledger#parentHash> "zQma45eMXmzKBXYwLdU7FvAEW3ekMy4fJjqEQVhYQFgwYAP" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmb6eicGxT6FAAZdxEzam2JpPu8ajiMJYhzPnhgHJJKh8f" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmc6b7weYQEu2NBDK9DB4HBc4bt2qQGbkvkEZBW6ajJ5F7" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmdxvSCwPjTvx3SAN2XHZ4uQpHKpbnHmns9BF8uZASW6Lx" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmePs3zy2fLPEsBXqGn2LPWSGYbzPy7CZTTz1f2ng3ysph" .
_:b0 <https://w3id.org/webledger#treeHash> "zQmPkZrQs9dyezAQkVniqkMjm5nP3cdWFBzNsnnFLrsNf9u" .
_:b2 <http://purl.org/dc/terms/created> "2018-12-21T23:40:20Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> _:b1 .
_:b2 <http://purl.org/dc/terms/creator> <https://bedrock.localhost:18443/consensus/continuity2017/voters/z6MkkabTusFkLnquxwHwCm28v59UX3P9Pn5scvc7fCaNvWUL> _:b1 .
_:b2 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2018> _:b1 .
_:b2 <https://w3id.org/security#jws> "eyJhbGciOiJFZERTQSIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..JJ5c7mF7ru9XhPtrNqj1s6J74yqOC0HcNyK_Wa0OcfDaiODZFIJ2dXIrc_qqqvTWynIqJid6yXkKsGAzyi_HDQ" _:b1 .
"#;

        b.iter(|| rdf_canonize::nquads::parse_nquads(&dataset_str))
    }

    #[bench]
    fn bench_veres_one_did_parse(b: &mut Bencher) {
        b.iter(|| rdf_canonize::nquads::parse_nquads(VERES_ONE_DID))
    }
}
