//! Canonical three-slot Grdfs14 construction and explicit admission boundaries.
#[path = "support/constructor.rs"]
mod constructor;
use constructor::apply_once;
use datalog_structural_model::{
    Atom, ConstantSymbol, DatalogCapabilities, DatalogFeature, DatalogProfile, EqualityMode,
    PredicateSymbol, Program, Rule, Term, VariableSymbol,
};
use rdf_reasoning::{Rdf12PropositionRuleId as R, TRIPLE_RELATION};

const TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const PROPERTY: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";
const RESOURCE: &str = "http://www.w3.org/2000/01/rdf-schema#Resource";
const PROPOSITION: &str = "http://www.w3.org/2000/01/rdf-schema#Proposition";

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
fn constructor_preserves_all_three_slots_including_generalized_predicates() {
    assert_eq!(R::ALL, &[R::Grdfs14]);
    assert_eq!(R::Grdfs14.name(), "grdfs14");
    for predicate in [key("urn:p"), key("literal:payload@en")] {
        let subject = key("urn:s");
        let object = Term::TripleTerm(Box::new(triple(key("urn:a"), key("urn:b"), key("urn:c"))));
        let snapshot = [
            triple(predicate.clone(), key(TYPE), key(PROPERTY)),
            triple(subject.clone(), key(TYPE), key(RESOURCE)),
            triple(object.clone(), key(TYPE), key(RESOURCE)),
        ];
        let result = apply_once(&R::Grdfs14.definition(), &snapshot).unwrap();
        // Two resources produce four ordered pairs in this immutable phase.
        assert_eq!(result.len(), 4);
        let quoted = triple(subject.clone(), predicate.clone(), object.clone());
        assert!(result.contains(&triple(
            Term::TripleTerm(Box::new(quoted.clone())),
            key(TYPE),
            key(PROPOSITION),
        )));
        assert!(!result.contains(&quoted), "quoted content is not asserted");
        assert_eq!(
            snapshot.len(),
            3,
            "constructor output does not feed this phase"
        );
    }
}

#[test]
fn missing_property_or_resource_evidence_does_not_construct_a_proposition() {
    let subject = key("urn:s");
    let predicate = key("urn:p");
    let object = key("urn:o");
    let snapshot = [
        triple(predicate.clone(), key(TYPE), key(PROPERTY)),
        triple(subject.clone(), key(TYPE), key(RESOURCE)),
        triple(object.clone(), key(TYPE), key(RESOURCE)),
    ];
    let wanted = triple(
        Term::TripleTerm(Box::new(triple(subject, predicate, object))),
        key(TYPE),
        key(PROPOSITION),
    );
    for missing in 0..3 {
        // bounded: two premises from a three-fact fixture.
        let partial: Vec<_> = snapshot
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != missing)
            .map(|(_, atom)| atom.clone())
            .collect();
        assert!(
            !apply_once(&R::Grdfs14.definition(), &partial)
                .unwrap()
                .contains(&wanted)
        );
    }
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Nested {
    id: R,
    program: Program,
}

#[test]
fn nested_transport_checks_safety_and_requires_explicit_triple_term_admission() {
    let value = Nested {
        id: R::Grdfs14,
        program: Program {
            rules: vec![R::Grdfs14.definition()],
        },
    };
    assert!(
        value
            .program
            .validate_for(&DatalogProfile::positive())
            .is_err()
    );
    let profile = DatalogProfile::new(
        DatalogCapabilities::positive().with(DatalogFeature::TripleTerms),
        EqualityMode::Default,
    );
    value.program.validate_for(&profile).unwrap();
    assert_eq!(
        serde_json::from_str::<Nested>(&serde_json::to_string(&value).unwrap()).unwrap(),
        value
    );
    assert_eq!(
        postcard::from_bytes::<Nested>(&postcard::to_allocvec(&value).unwrap()).unwrap(),
        value
    );
    assert_eq!(serde_json::to_string(&value.id).unwrap(), r#""grdfs14""#);
    let mut unsafe_value = value;
    let Term::TripleTerm(inner) = &mut unsafe_value.program.rules[0]
        .head
        .as_atom_mut()
        .expect("single-atom head")
        .terms[0]
    else {
        panic!("canonical quoted head")
    };
    inner.terms[1] = Term::Variable(VariableSymbol::new("unbound_predicate"));
    assert!(
        serde_json::from_str::<Nested>(&serde_json::to_string(&unsafe_value).unwrap()).is_err()
    );
    assert!(
        postcard::from_bytes::<Nested>(&postcard::to_allocvec(&unsafe_value).unwrap()).is_err()
    );
    assert!(serde_json::from_str::<R>(r#""unknown""#).is_err());
}

#[test]
fn substituted_nesting_overflow_is_an_explicit_error() {
    let mut nested = key("urn:leaf");
    for _ in 0..4 {
        nested = Term::TripleTerm(Box::new(triple(key("urn:a"), key("urn:p"), nested)));
    }
    let resource = triple(nested, key(TYPE), key(RESOURCE));
    Rule::try_new(resource.clone(), vec![]).unwrap();
    let snapshot = [triple(key("urn:p"), key(TYPE), key(PROPERTY)), resource];
    assert!(apply_once(&R::Grdfs14.definition(), &snapshot).is_err());
}
