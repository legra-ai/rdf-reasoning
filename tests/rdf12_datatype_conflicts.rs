//! Positive evidence for the three disjoint mandatory default datatype classes.
#[path = "support/relations.rs"]
mod relations;
use datalog_structural_model::{DatalogProfile, Program, Term, VariableSymbol};
use rdf_reasoning::{
    Rdf12Datatype as D, Rdf12DatatypeConflictId as C, Rdf12ExistentialAxiomId as E, RuleId,
    TRIPLE_RELATION,
};
use relations::{Fact, closure};

const TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const SUBCLASS: &str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";

#[test]
fn exactly_three_canonical_pairs_report_the_same_subject_with_both_premises() {
    let cases = [
        (
            C::StringLangString,
            [D::String, D::LangString],
            "string-lang-string",
        ),
        (
            C::StringDirLangString,
            [D::String, D::DirLangString],
            "string-dir-lang-string",
        ),
        (
            C::LangStringDirLangString,
            [D::LangString, D::DirLangString],
            "lang-string-dir-lang-string",
        ),
    ];
    assert_eq!(C::ALL.len(), cases.len());
    for ((id, pair, name), listed) in cases.into_iter().zip(C::ALL) {
        assert_eq!(&id, listed);
        assert_eq!(id.datatypes(), pair);
        assert_eq!(id.name(), name);
        assert_eq!(
            id.witness_relation(),
            format!("urn:rdf-reasoning:rdf12-violation:{name}")
        );
        let rules = [id.definition()];
        assert_eq!(rules[0].body.len(), 2);
        for subject in [
            "iri:source/member",
            "blank:source/member",
            "literal:exact/member",
        ] {
            let left = Fact::new(TRIPLE_RELATION, &[subject, TYPE, pair[0].iri()]);
            let right = Fact::new(TRIPLE_RELATION, &[subject, TYPE, pair[1].iri()]);
            for input in [[left.clone(), right.clone()], [right, left]] {
                let result = closure(&rules, &input);
                assert!(result.contains(&Fact::new(id.witness_relation(), &[subject])));
                assert_eq!(
                    result.len(),
                    3,
                    "one non-RDF diagnostic, both original premises"
                );
            }
        }
    }
}

#[test]
fn missing_or_different_subjects_and_opaque_datatypes_do_not_report_conflicts() {
    for id in C::ALL {
        let [left, right] = id.datatypes();
        let rules = [id.definition()];
        let missing = [Fact::new(TRIPLE_RELATION, &["member", TYPE, left.iri()])];
        assert_eq!(closure(&rules, &missing).len(), 1);
        assert!(closure(&rules, &[]).is_empty());
        let distinct = [
            Fact::new(
                TRIPLE_RELATION,
                &["literal:source/lexical@EN", TYPE, left.iri()],
            ),
            Fact::new(
                TRIPLE_RELATION,
                &["literal:source/lexical@en/other", TYPE, right.iri()],
            ),
        ];
        assert_eq!(
            closure(&rules, &distinct).len(),
            2,
            "no guessed value equality"
        );
        let opaque = [
            missing[0].clone(),
            Fact::new(TRIPLE_RELATION, &["member", TYPE, "urn:datatype:opaque"]),
            Fact::new(
                TRIPLE_RELATION,
                &[
                    "urn:datatype:opaque",
                    TYPE,
                    "http://www.w3.org/2000/01/rdf-schema#Datatype",
                ],
            ),
            Fact::new(
                TRIPLE_RELATION,
                &["member", id.witness_relation(), right.iri()],
            ),
        ];
        assert_eq!(closure(&rules, &opaque).len(), 4);
    }
}

#[test]
fn nonempty_witnesses_expose_contradictory_subclass_constraints_without_literal_data() {
    for id in C::ALL {
        let [left, right] = id.datatypes();
        let rules = [RuleId::Rdfs9.definition(), id.definition()];
        let subclass = Fact::new(TRIPLE_RELATION, &[left.iri(), SUBCLASS, right.iri()]);
        assert_eq!(closure(&rules, std::slice::from_ref(&subclass)).len(), 1);
        // The executor supplies this source-scoped blank for the shared descriptor.
        let witness = "blank:attempt/source/nonempty-left";
        let input = [
            subclass,
            Fact::new(
                TRIPLE_RELATION,
                &[witness, TYPE, E::RdfD1a(left).class_iri()],
            ),
        ];
        let result = closure(&rules, &input);
        assert!(result.contains(&Fact::new(TRIPLE_RELATION, &[witness, TYPE, right.iri()])));
        assert!(result.contains(&Fact::new(id.witness_relation(), &[witness])));
        assert_eq!(result.len(), 4);
    }
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Nested {
    id: C,
    program: Program,
}

#[test]
fn checked_nested_rules_preserve_pair_identity_and_reject_unsafe_heads() {
    for id in C::ALL {
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
            Term::Variable(VariableSymbol::new("unbound"));
        assert!(
            serde_json::from_str::<Nested>(&serde_json::to_string(&unsafe_value).unwrap()).is_err()
        );
        assert!(
            postcard::from_bytes::<Nested>(&postcard::to_allocvec(&unsafe_value).unwrap()).is_err()
        );
    }
    assert!(serde_json::from_str::<C>(r#""string-integer""#).is_err());
    assert!(postcard::from_bytes::<C>(&[3]).is_err());
}
