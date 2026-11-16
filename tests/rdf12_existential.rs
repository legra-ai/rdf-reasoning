//! Finite semantic identities for nonempty default datatypes and propositions.
use rdf_reasoning::{Rdf12Datatype as D, Rdf12ExistentialAxiomId as E};

#[test]
fn mandatory_nonempty_classes_have_exact_distinct_semantic_identities() {
    const PROPOSITION: &str = "http://www.w3.org/2000/01/rdf-schema#Proposition";
    assert_eq!(
        E::ALL,
        &[
            E::RdfD1a(D::String),
            E::RdfD1a(D::LangString),
            E::RdfD1a(D::DirLangString),
            E::Rdfs14a,
        ]
    );
    for (index, id) in E::ALL.iter().enumerate() {
        if let E::RdfD1a(datatype) = id {
            assert_eq!(id.name(), "rdfD1a");
            assert_eq!(id.class_iri(), datatype.iri());
        } else {
            assert_eq!(id.name(), "rdfs14a");
            assert_eq!(id.class_iri(), PROPOSITION);
        }
        for other in &E::ALL[index + 1..] {
            assert_ne!(id, other);
            assert_ne!(id.class_iri(), other.class_iri());
        }
    }
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Nested {
    requirement: E,
}

#[test]
fn nested_json_and_binary_preserve_clause_and_datatype_parameters() {
    for id in E::ALL {
        let value = Nested { requirement: *id };
        assert_eq!(
            serde_json::from_str::<Nested>(&serde_json::to_string(&value).unwrap()).unwrap(),
            value
        );
        assert_eq!(
            postcard::from_bytes::<Nested>(&postcard::to_allocvec(&value).unwrap()).unwrap(),
            value
        );
        let json = match id {
            E::RdfD1a(datatype) => format!(r#"{{"rdfD1a":"{}"}}"#, datatype.iri()),
            E::Rdfs14a => r#""rdfs14a""#.to_owned(),
        };
        assert_eq!(serde_json::to_string(id).unwrap(), json);
    }
}

#[test]
fn requested_unknown_datatypes_and_forged_descriptor_shapes_fail() {
    for input in [
        r#"{"rdfD1a":"urn:datatype:opaque"}"#,
        r#"{"rdfD1a":"http://www.w3.org/2001/XMLSchema#integer"}"#,
        r#"{"rdfD1a":null}"#,
        r#"{"rdfD1a":"xsd:string"}"#,
        r#"{"rdfs14a":"urn:fixed-witness"}"#,
        r#""unknown""#,
    ] {
        assert!(serde_json::from_str::<E>(input).is_err(), "{input}");
    }
    // Unknown enum and nested datatype tags must also fail in binary input.
    assert!(postcard::from_bytes::<E>(&[2]).is_err());
    assert!(postcard::from_bytes::<E>(&[0, 3]).is_err());
}
