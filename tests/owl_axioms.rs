//! Exact OWL 2 RL/RDF 2012 empty-body clauses, independent of execution.
use datalog_structural_model::{DatalogProfile, Program, Rule, Term, VariableSymbol};
use rdf_reasoning::{OwlRlAxiomId as A, Preset, TRIPLE_RELATION};

const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
const RDFS: &str = "http://www.w3.org/2000/01/rdf-schema#";
const OWL: &str = "http://www.w3.org/2002/07/owl#";
const XSD: &str = "http://www.w3.org/2001/XMLSchema#";

fn subjects(id: A, object: &str) -> Vec<String> {
    // bounded: at most 32 finite standard datatype declarations.
    id.definitions()
        .into_iter()
        .map(|rule| {
            assert!(rule.body.is_empty());
            assert_eq!(rule.head.predicate.as_str(), TRIPLE_RELATION);
            assert_eq!(rule.head.terms.len(), 3);
            let values: Vec<_> = rule
                .head
                .terms
                .iter()
                .map(|term| match term {
                    Term::Constant(value) => value.as_str(),
                    _ => panic!("axiom must contain only IRI constants"),
                })
                .collect();
            assert_eq!(values[1], format!("{RDF}type"));
            assert_eq!(values[2], object);
            values[0].to_owned()
        })
        .collect()
}

#[test]
fn annotation_seeds_are_exactly_the_nine_standard_properties() {
    let mut actual = subjects(A::PrpAp, &format!("{OWL}AnnotationProperty"));
    // bounded: exactly nine built-in annotation properties.
    let mut expected: Vec<_> = [
        (RDFS, "label"),
        (RDFS, "comment"),
        (RDFS, "seeAlso"),
        (RDFS, "isDefinedBy"),
        (OWL, "deprecated"),
        (OWL, "versionInfo"),
        (OWL, "priorVersion"),
        (OWL, "backwardCompatibleWith"),
        (OWL, "incompatibleWith"),
    ]
    .into_iter()
    .map(|(namespace, local)| format!("{namespace}{local}"))
    .collect();
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn class_seeds_do_not_assert_an_instance_of_nothing() {
    assert_eq!(
        subjects(A::ClsThing, &format!("{OWL}Class")),
        [format!("{OWL}Thing")]
    );
    assert_eq!(
        subjects(A::ClsNothing1, &format!("{OWL}Class")),
        [format!("{OWL}Nothing")]
    );
}

#[test]
fn datatype_seeds_cover_the_complete_pinned_rl_map_and_no_extensions() {
    // bounded: exact 32-type list in OWL 2 Profiles 2012 section 4.2.1.
    let mut expected: Vec<_> = [
        "decimal",
        "integer",
        "nonNegativeInteger",
        "nonPositiveInteger",
        "positiveInteger",
        "negativeInteger",
        "long",
        "int",
        "short",
        "byte",
        "unsignedLong",
        "unsignedInt",
        "unsignedShort",
        "unsignedByte",
        "float",
        "double",
        "string",
        "normalizedString",
        "token",
        "language",
        "Name",
        "NCName",
        "NMTOKEN",
        "boolean",
        "hexBinary",
        "base64Binary",
        "anyURI",
        "dateTime",
        "dateTimeStamp",
    ]
    .into_iter()
    .map(|local| format!("{XSD}{local}"))
    .chain([
        format!("{RDF}PlainLiteral"),
        format!("{RDF}XMLLiteral"),
        format!("{RDFS}Literal"),
    ])
    .collect();
    let mut actual = subjects(A::DtType1, &format!("{RDFS}Datatype"));
    expected.sort();
    actual.sort();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 32);
    for absent in [
        format!("{OWL}real"),
        format!("{OWL}rational"),
        format!("{RDF}langString"),
    ] {
        assert!(!actual.contains(&absent));
    }
}

#[test]
fn all_axioms_validate_and_round_trip_without_changing_presets() {
    assert_eq!(A::ALL.len(), 4);
    for id in A::ALL {
        let rules = id.definitions();
        let program = Program {
            rules: rules.clone(),
        };
        program.validate_for(&DatalogProfile::positive()).unwrap();
        assert_eq!(
            serde_json::from_str::<A>(&serde_json::to_string(id).unwrap()).unwrap(),
            *id
        );
        assert_eq!(
            postcard::from_bytes::<A>(&postcard::to_allocvec(id).unwrap()).unwrap(),
            *id
        );
        for rule in rules {
            assert_eq!(
                serde_json::from_str::<Rule>(&serde_json::to_string(&rule).unwrap()).unwrap(),
                rule
            );
            assert_eq!(
                postcard::from_bytes::<Rule>(&postcard::to_allocvec(&rule).unwrap()).unwrap(),
                rule
            );
        }
    }
    assert!(serde_json::from_str::<A>("\"unknown\"").is_err());
    assert!(serde_json::from_str::<A>("{\"prp-ap\":{\"extra\":true}}").is_err());
    assert_eq!(Preset::RdfsOwl2RlRelationalCore.rules().len(), 60);
    assert!(Preset::ALL.iter().all(|preset| {
        preset
            .program()
            .rules
            .iter()
            .all(|rule| !rule.body.is_empty())
    }));
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct NestedAxiom {
    id: A,
    program: Program,
}

#[test]
fn nested_transport_rechecks_ground_fact_safety() {
    for id in A::ALL {
        let value = NestedAxiom {
            id: *id,
            program: Program {
                rules: id.definitions(),
            },
        };
        assert_eq!(
            serde_json::from_str::<NestedAxiom>(&serde_json::to_string(&value).unwrap()).unwrap(),
            value
        );
        assert_eq!(
            postcard::from_bytes::<NestedAxiom>(&postcard::to_allocvec(&value).unwrap()).unwrap(),
            value
        );
        let mut malformed = value;
        malformed.program.rules[0].head.terms[0] = Term::Variable(VariableSymbol::new("unbound"));
        assert!(
            serde_json::from_str::<NestedAxiom>(&serde_json::to_string(&malformed).unwrap())
                .is_err()
        );
        assert!(
            postcard::from_bytes::<NestedAxiom>(&postcard::to_allocvec(&malformed).unwrap())
                .is_err()
        );
    }
}
