//! Source and target-context semantic cases for trusted occurrence relations.
#[path = "support/constructor.rs"]
mod constructor;
#[path = "support/occurrences.rs"]
mod occurrences;
use constructor::apply_once;
use datalog_structural_model::{
    Atom, ConstantSymbol, DatalogProfile, PredicateSymbol, Program, Term, VariableSymbol,
};
use occurrences::occurrences;
use rdf_reasoning::{
    RDF12_APPEARING_TERM_RELATION as TERMS, RDF12_APPEARING_TRIPLE_RELATION as TRIPLES,
    Rdf12OccurrenceRuleId as R, TRIPLE_RELATION,
};

const TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const PROPERTY: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";
const RESOURCE: &str = "http://www.w3.org/2000/01/rdf-schema#Resource";

fn key(value: &str) -> Term {
    Term::Constant(ConstantSymbol::new(value))
}
fn triple(subject: Term, predicate: Term, object: Term) -> Atom {
    Atom {
        predicate: PredicateSymbol::new(TRIPLE_RELATION),
        terms: vec![subject, predicate, object],
    }
}

#[test]
fn nested_source_occurrences_type_predicates_and_all_terms_without_asserting_contents() {
    assert_eq!(R::ALL, &[R::RdfD2, R::Rdfs4]);
    assert_eq!(R::RdfD2.name(), "rdfD2");
    assert_eq!(R::Rdfs4.name(), "rdfs4");
    let blank = key("blank:source/x");
    let literal = key("literal:source/exact^^urn:opaque-datatype");
    let inner = triple(blank.clone(), key("urn:inner-p"), literal.clone());
    let inner_term = Term::TripleTerm(Box::new(inner.clone()));
    let middle = triple(key("urn:middle-s"), key("urn:middle-p"), inner_term.clone());
    let middle_term = Term::TripleTerm(Box::new(middle.clone()));
    let root = triple(blank.clone(), key("urn:outer-p"), middle_term.clone());
    let snapshot = occurrences(&root);
    let properties = apply_once(&R::RdfD2.definition(), &snapshot).unwrap();
    for predicate in ["urn:outer-p", "urn:middle-p", "urn:inner-p"] {
        assert!(properties.contains(&triple(key(predicate), key(TYPE), key(PROPERTY))));
    }
    assert_eq!(properties.len(), 3);
    let resources = apply_once(&R::Rdfs4.definition(), &snapshot).unwrap();
    for term in [
        blank,
        literal,
        inner_term,
        middle_term,
        key("urn:middle-s"),
        key("urn:outer-p"),
        key("urn:middle-p"),
        key("urn:inner-p"),
    ] {
        assert!(resources.contains(&triple(term, key(TYPE), key(RESOURCE))));
    }
    assert!(
        !resources.contains(&triple(
            key("urn:opaque-datatype"),
            key(TYPE),
            key(RESOURCE)
        )),
        "literal metadata is not an appearing term"
    );
    for output in properties.iter().chain(&resources) {
        assert_ne!(output, &inner);
        assert_ne!(output, &middle);
        assert_ne!(output, &root);
    }
}

#[test]
fn target_context_resource_typing_preserves_scoped_nested_blanks_without_property_seeding() {
    let source_blank = key("blank:source/x");
    let target_blank = key("blank:target/x");
    let nested = triple(
        target_blank.clone(),
        key("urn:target-inner-p"),
        key("urn:target-o"),
    );
    let root = triple(
        target_blank.clone(),
        key("urn:target-p"),
        Term::TripleTerm(Box::new(nested)),
    );
    let target_context = occurrences(&root);
    // Appendix A target phase selects rdfs4, never rdfD2 or asserted target tuples.
    let resources = apply_once(&R::Rdfs4.definition(), &target_context).unwrap();
    assert!(resources.contains(&triple(target_blank, key(TYPE), key(RESOURCE))));
    assert!(!resources.contains(&triple(source_blank, key(TYPE), key(RESOURCE))));
    for predicate in ["urn:target-p", "urn:target-inner-p"] {
        assert!(resources.contains(&triple(key(predicate), key(TYPE), key(RESOURCE))));
        assert!(!resources.contains(&triple(key(predicate), key(TYPE), key(PROPERTY))));
    }
    assert!(!resources.contains(&root));
}

#[test]
fn user_rdf_cannot_forge_occurrences_and_missing_relations_have_no_consequences() {
    let forged = [
        triple(key("urn:x"), key(TERMS), key("urn:o")),
        triple(key("urn:x"), key(TRIPLES), key("urn:o")),
    ];
    for id in R::ALL {
        let rule = id.definition();
        assert!(apply_once(&rule, &forged).unwrap().is_empty());
        assert!(apply_once(&rule, &[]).unwrap().is_empty());
    }
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Nested {
    id: R,
    program: Program,
}

#[test]
fn nested_transport_preserves_occurrence_definitions_and_checks_head_safety() {
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
            format!("\"{}\"", id.name())
        );
        assert_eq!(
            serde_json::from_str::<Nested>(&serde_json::to_string(&value).unwrap()).unwrap(),
            value
        );
        assert_eq!(
            postcard::from_bytes::<Nested>(&postcard::to_allocvec(&value).unwrap()).unwrap(),
            value
        );
        let mut unsafe_value = value;
        unsafe_value.program.rules[0].head.terms[0] =
            Term::Variable(VariableSymbol::new("missing"));
        assert!(
            serde_json::from_str::<Nested>(&serde_json::to_string(&unsafe_value).unwrap()).is_err()
        );
        assert!(
            postcard::from_bytes::<Nested>(&postcard::to_allocvec(&unsafe_value).unwrap()).is_err()
        );
    }
    assert!(serde_json::from_str::<R>(r#""rdfD1""#).is_err());
    assert!(postcard::from_bytes::<R>(&[2]).is_err());
}
