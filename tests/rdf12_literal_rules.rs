//! Checked default-map GrdfD1 definitions over explicit metadata premises.
#[path = "support/relations.rs"]
mod relations;
use datalog_structural_model::{DatalogProfile, Program, Term, VariableSymbol};
use rdf_reasoning::{
    RDF12_LITERAL_DATATYPE_RELATION as META, Rdf12Datatype as D, Rdf12LiteralRuleId as R,
    TRIPLE_RELATION,
};
use relations::{Fact, closure};

const TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const DATATYPE: &str = "http://www.w3.org/2000/01/rdf-schema#Datatype";

#[test]
fn every_default_literal_rule_preserves_the_exact_literal_key_and_datatype() {
    assert_eq!(
        R::ALL,
        &[
            R::GrdfD1(D::String),
            R::GrdfD1(D::LangString),
            R::GrdfD1(D::DirLangString)
        ]
    );
    for id in R::ALL {
        assert_eq!(id.name(), "grdf-d1");
        let datatype = id.datatype();
        let input = [Fact::new(META, &["literal:scoped/exact", datatype.iri()])];
        let rules = [id.definition()];
        let result = closure(&rules, &input);
        assert!(result.contains(&Fact::new(
            TRIPLE_RELATION,
            &["literal:scoped/exact", TYPE, datatype.iri()]
        )));
        assert_eq!(result.len(), 2);
        assert_eq!(rules[0].body.len(), 1);
        assert!(closure(&rules, &[]).is_empty());
        for other in D::ALL {
            if other == &datatype {
                continue;
            }
            assert!(!result.contains(&Fact::new(
                TRIPLE_RELATION,
                &["literal:scoped/exact", TYPE, other.iri()]
            )));
            let wrong = [Fact::new(META, &["literal:scoped/exact", other.iri()])];
            assert_eq!(closure(&rules, &wrong).len(), 1);
        }
    }
}

#[test]
fn graph_datatype_declarations_do_not_enable_unsupported_recognition() {
    const UNKNOWN: &str = "urn:datatype:opaque";
    // bounded: three authored rules and five independent facts.
    let rules: Vec<_> = R::ALL.iter().map(|id| id.definition()).collect();
    let input = [
        Fact::new(META, &["unknown-literal", UNKNOWN]),
        Fact::new(TRIPLE_RELATION, &[UNKNOWN, TYPE, DATATYPE]),
        Fact::new(TRIPLE_RELATION, &["source", "predicate", "unknown-literal"]),
        Fact::new(
            "urn:unrelated-metadata",
            &["other-literal", D::String.iri()],
        ),
        // User RDF using the auxiliary IRI as its predicate cannot forge metadata.
        Fact::new(
            TRIPLE_RELATION,
            &["injected-literal", META, D::String.iri()],
        ),
    ];
    assert_eq!(closure(&rules, &input).len(), input.len());
}

#[test]
fn selected_occurrence_context_does_not_change_source_term_identity() {
    let rule = R::GrdfD1(D::LangString).definition();
    for key in [
        "source:lexical@EN",
        "target:lexical@en",
        "nested:lexical@EN",
    ] {
        // The adapter selects source/target/recursive occurrences; no graph anchor is required.
        let result = closure(
            std::slice::from_ref(&rule),
            &[Fact::new(META, &[key, D::LangString.iri()])],
        );
        assert!(result.contains(&Fact::new(
            TRIPLE_RELATION,
            &[key, TYPE, D::LangString.iri()]
        )));
        assert_eq!(result.len(), 2);
    }
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Nested {
    id: R,
    program: Program,
}

#[test]
fn checked_nested_metadata_rules_and_identifiers_round_trip() {
    for id in R::ALL {
        let value = Nested {
            id: *id,
            program: Program {
                rules: vec![id.definition()],
            },
        };
        value
            .program
            .validate_for(&DatalogProfile::positive())
            .unwrap();
        assert_eq!(
            serde_json::to_string(id).unwrap(),
            format!(r#"{{"grdf-d1":"{}"}}"#, id.datatype().iri())
        );
        assert_eq!(
            serde_json::from_str::<Nested>(&serde_json::to_string(&value).unwrap()).unwrap(),
            value
        );
        assert_eq!(
            postcard::from_bytes::<Nested>(&postcard::to_allocvec(&value).unwrap()).unwrap(),
            value
        );
        let mut unsafe_program = value;
        unsafe_program.program.rules[0]
            .head
            .as_atom_mut()
            .expect("single-atom head")
            .terms[0] = Term::Variable(VariableSymbol::new("unbound"));
        assert!(
            serde_json::from_str::<Nested>(&serde_json::to_string(&unsafe_program).unwrap())
                .is_err()
        );
        assert!(
            postcard::from_bytes::<Nested>(&postcard::to_allocvec(&unsafe_program).unwrap())
                .is_err()
        );
    }
    assert!(serde_json::from_str::<R>(r#"{"grdf-d1":"urn:datatype:opaque"}"#).is_err());
    assert!(
        serde_json::from_str::<R>(r#"{"unknown":"http://www.w3.org/2001/XMLSchema#string"}"#)
            .is_err()
    );
}
