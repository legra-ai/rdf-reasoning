//! Independent inventories from RDF 1.1 Semantics (2014), sections 8 and 9.
use datalog_structural_model::{DatalogProfile, Program, Rule, Term};
use rdf_reasoning::{Rdf11AxiomId as A, Rdf11ContainerProperty as Container, TRIPLE_RELATION};
use std::collections::BTreeSet;

fn expand(value: &str) -> String {
    let (namespace, local) = value.split_once(':').unwrap();
    format!(
        "{}{local}",
        match namespace {
            "rdf" => "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
            "rdfs" => "http://www.w3.org/2000/01/rdf-schema#",
            "xsd" => "http://www.w3.org/2001/XMLSchema#",
            _ => panic!("unknown fixture namespace"),
        }
    )
}

fn triple(rule: Rule) -> [String; 3] {
    assert!(rule.body.is_empty());
    assert_eq!(rule.head.predicate.as_str(), TRIPLE_RELATION);
    assert_eq!(rule.head.terms.len(), 3);
    std::array::from_fn(|index| match &rule.head.terms[index] {
        Term::Constant(value) => value.as_str().to_owned(),
        _ => panic!("axioms must be ground"),
    })
}

fn inventory(id: A, expected: &[[&str; 3]]) {
    // bounded: at most 38 fixed standard vocabulary triples, not caller facts.
    let actual: BTreeSet<_> = id.definitions().into_iter().map(triple).collect();
    let expected: BTreeSet<_> = expected.iter().map(|row| row.map(expand)).collect();
    assert_eq!(actual.len(), id.definitions().len(), "no duplicate axioms");
    assert_eq!(actual, expected);
}

#[test]
fn finite_rdf_axioms_are_exact() {
    inventory(
        A::Rdf,
        &[
            ["rdf:type", "rdf:type", "rdf:Property"],
            ["rdf:subject", "rdf:type", "rdf:Property"],
            ["rdf:predicate", "rdf:type", "rdf:Property"],
            ["rdf:object", "rdf:type", "rdf:Property"],
            ["rdf:first", "rdf:type", "rdf:Property"],
            ["rdf:rest", "rdf:type", "rdf:Property"],
            ["rdf:value", "rdf:type", "rdf:Property"],
            ["rdf:nil", "rdf:type", "rdf:List"],
        ],
    );
}

#[test]
fn finite_rdfs_axioms_are_exact_and_exclude_the_infinite_container_family() {
    inventory(
        A::Rdfs,
        &[
            ["rdf:type", "rdfs:domain", "rdfs:Resource"],
            ["rdfs:domain", "rdfs:domain", "rdf:Property"],
            ["rdfs:range", "rdfs:domain", "rdf:Property"],
            ["rdfs:subPropertyOf", "rdfs:domain", "rdf:Property"],
            ["rdfs:subClassOf", "rdfs:domain", "rdfs:Class"],
            ["rdf:subject", "rdfs:domain", "rdf:Statement"],
            ["rdf:predicate", "rdfs:domain", "rdf:Statement"],
            ["rdf:object", "rdfs:domain", "rdf:Statement"],
            ["rdfs:member", "rdfs:domain", "rdfs:Resource"],
            ["rdf:first", "rdfs:domain", "rdf:List"],
            ["rdf:rest", "rdfs:domain", "rdf:List"],
            ["rdfs:seeAlso", "rdfs:domain", "rdfs:Resource"],
            ["rdfs:isDefinedBy", "rdfs:domain", "rdfs:Resource"],
            ["rdfs:comment", "rdfs:domain", "rdfs:Resource"],
            ["rdfs:label", "rdfs:domain", "rdfs:Resource"],
            ["rdf:value", "rdfs:domain", "rdfs:Resource"],
            ["rdf:type", "rdfs:range", "rdfs:Class"],
            ["rdfs:domain", "rdfs:range", "rdfs:Class"],
            ["rdfs:range", "rdfs:range", "rdfs:Class"],
            ["rdfs:subPropertyOf", "rdfs:range", "rdf:Property"],
            ["rdfs:subClassOf", "rdfs:range", "rdfs:Class"],
            ["rdf:subject", "rdfs:range", "rdfs:Resource"],
            ["rdf:predicate", "rdfs:range", "rdfs:Resource"],
            ["rdf:object", "rdfs:range", "rdfs:Resource"],
            ["rdfs:member", "rdfs:range", "rdfs:Resource"],
            ["rdf:first", "rdfs:range", "rdfs:Resource"],
            ["rdf:rest", "rdfs:range", "rdf:List"],
            ["rdfs:seeAlso", "rdfs:range", "rdfs:Resource"],
            ["rdfs:isDefinedBy", "rdfs:range", "rdfs:Resource"],
            ["rdfs:comment", "rdfs:range", "rdfs:Literal"],
            ["rdfs:label", "rdfs:range", "rdfs:Literal"],
            ["rdf:value", "rdfs:range", "rdfs:Resource"],
            ["rdf:Alt", "rdfs:subClassOf", "rdfs:Container"],
            ["rdf:Bag", "rdfs:subClassOf", "rdfs:Container"],
            ["rdf:Seq", "rdfs:subClassOf", "rdfs:Container"],
            [
                "rdfs:ContainerMembershipProperty",
                "rdfs:subClassOf",
                "rdf:Property",
            ],
            ["rdfs:isDefinedBy", "rdfs:subPropertyOf", "rdfs:seeAlso"],
            ["rdfs:Datatype", "rdfs:subClassOf", "rdfs:Class"],
        ],
    );
}

#[test]
fn default_datatype_declarations_do_not_recognize_other_encountered_types() {
    inventory(
        A::DefaultDatatypes,
        &[
            ["xsd:string", "rdf:type", "rdfs:Datatype"],
            ["rdf:langString", "rdf:type", "rdfs:Datatype"],
        ],
    );
}

#[test]
fn one_container_iri_generates_only_its_four_axioms_without_integer_overflow() {
    for local in [
        "_1",
        "_937",
        "_99999999999999999999999999999999999999999999",
    ] {
        let iri = expand(&format!("rdf:{local}"));
        let property = Container::try_new(iri.as_str()).unwrap();
        assert_eq!(property.iri(), iri);
        assert_eq!(
            property.iri().as_ptr(),
            iri.as_ptr(),
            "borrow without copying input"
        );
        assert_eq!(
            triple(property.rdf_definition()),
            [iri.clone(), expand("rdf:type"), expand("rdf:Property")]
        );
        let [kind, domain, range] = property.rdfs_definitions().map(triple);
        assert_eq!(
            kind,
            [
                iri.clone(),
                expand("rdf:type"),
                expand("rdfs:ContainerMembershipProperty")
            ]
        );
        assert_eq!(
            domain,
            [iri.clone(), expand("rdfs:domain"), expand("rdfs:Resource")]
        );
        assert_eq!(range, [iri, expand("rdfs:range"), expand("rdfs:Resource")]);
    }
    for local in [
        "_", "_0", "_01", "_+1", "_-1", "_1e3", "_1a", "_١", "_１", "_ 1", "_1 ",
    ] {
        assert!(
            Container::try_new(expand(&format!("rdf:{local}"))).is_err(),
            "{local}"
        );
    }
    for other in [
        "urn:other:_1",
        "rdf:_1",
        "",
        "http://www.w3.org/2000/01/rdf-schema#_1",
    ] {
        assert!(Container::try_new(other).is_err());
    }
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Nested {
    family: A,
    container: Container,
    program: Program,
}

#[test]
fn checked_seed_types_and_programs_round_trip_in_nested_transport() {
    assert_eq!(A::ALL.len(), 3);
    for family in A::ALL {
        let value = Nested {
            family: *family,
            container: Container::try_new(expand("rdf:_123")).unwrap(),
            program: Program {
                rules: family.definitions(),
            },
        };
        value
            .program
            .validate_for(&DatalogProfile::positive())
            .unwrap();
        assert_eq!(
            serde_json::from_str::<Nested>(&serde_json::to_string(&value).unwrap()).unwrap(),
            value
        );
        assert_eq!(
            postcard::from_bytes::<Nested>(&postcard::to_allocvec(&value).unwrap()).unwrap(),
            value
        );
    }
    assert!(serde_json::from_str::<A>("\"unknown\"").is_err());
    assert!(
        serde_json::from_str::<Container>(
            r#"{"iri":"http://www.w3.org/1999/02/22-rdf-syntax-ns#_0"}"#
        )
        .is_err()
    );
    assert!(
        serde_json::from_str::<Container>(
            r#"{"iri":"http://www.w3.org/1999/02/22-rdf-syntax-ns#_1","extra":true}"#
        )
        .is_err()
    );
}
