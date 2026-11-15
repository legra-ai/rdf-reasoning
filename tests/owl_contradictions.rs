//! Independent finite witnesses for seven flat OWL 2 RL/RDF contradictions.
#[path = "support/relations.rs"]
mod relations;

use datalog_structural_model::{DatalogProfile, Literal, Program, Term, VariableSymbol};
use rdf_reasoning::{OwlContradictionRuleId as C, Preset, TRIPLE_RELATION};
use relations::{Fact, closure};

struct Case {
    id: C,
    name: &'static str,
    input: &'static [[&'static str; 3]],
    expected: &'static [&'static str],
}

// bounded: seven independent cases, at most three triples and four operands each.
const CASES: &[Case] = &[
    Case {
        id: C::EqDiff1,
        name: "eq-diff1",
        input: &[["a", "owl:sameAs", "b"], ["a", "owl:differentFrom", "b"]],
        expected: &["a", "b"],
    },
    Case {
        id: C::PrpIrp,
        name: "prp-irp",
        input: &[
            ["p", "rdf:type", "owl:IrreflexiveProperty"],
            ["a", "p", "a"],
        ],
        expected: &["p", "a"],
    },
    Case {
        id: C::PrpAsyp,
        name: "prp-asyp",
        input: &[
            ["p", "rdf:type", "owl:AsymmetricProperty"],
            ["a", "p", "b"],
            ["b", "p", "a"],
        ],
        expected: &["p", "a", "b"],
    },
    Case {
        id: C::PrpPdw,
        name: "prp-pdw",
        input: &[
            ["p", "owl:propertyDisjointWith", "q"],
            ["a", "p", "b"],
            ["a", "q", "b"],
        ],
        expected: &["p", "q", "a", "b"],
    },
    Case {
        id: C::CaxDw,
        name: "cax-dw",
        input: &[
            ["A", "owl:disjointWith", "B"],
            ["a", "rdf:type", "A"],
            ["a", "rdf:type", "B"],
        ],
        expected: &["A", "B", "a"],
    },
    Case {
        id: C::ClsCom,
        name: "cls-com",
        input: &[
            ["A", "owl:complementOf", "B"],
            ["a", "rdf:type", "A"],
            ["a", "rdf:type", "B"],
        ],
        expected: &["A", "B", "a"],
    },
    Case {
        id: C::ClsNothing2,
        name: "cls-nothing2",
        input: &[["a", "rdf:type", "owl:Nothing"]],
        expected: &["a"],
    },
];

fn expand(value: &str) -> String {
    for (prefix, namespace) in [
        ("rdf:", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
        ("owl:", "http://www.w3.org/2002/07/owl#"),
    ] {
        if let Some(local) = value.strip_prefix(prefix) {
            return format!("{namespace}{local}");
        }
    }
    value.to_owned()
}

fn fact(row: &[&str; 3]) -> Fact {
    Fact(
        TRIPLE_RELATION.to_owned(),
        row.iter().map(|term| expand(term)).collect(),
    )
}

#[test]
fn every_contradiction_emits_only_its_identified_non_rdf_witness() {
    assert_eq!(C::ALL.len(), CASES.len());
    for case in CASES {
        assert!(C::ALL.contains(&case.id));
        assert_eq!(case.id.name(), case.name);
        assert_eq!(
            case.id.witness_relation(),
            format!("urn:rdf-reasoning:owl-violation:{}", case.name)
        );
        let rule = case.id.definition();
        Program {
            rules: vec![rule.clone()],
        }
        .validate_for(&DatalogProfile::positive())
        .unwrap();
        assert!(
            rule.body
                .iter()
                .all(|body| matches!(body, Literal::Positive(_)))
        );
        // bounded: at most three independently authored fixture triples.
        let input: Vec<_> = case.input.iter().map(fact).collect();
        let result = closure(&[rule], &input);
        let witness = Fact::new(case.id.witness_relation(), case.expected);
        assert!(result.contains(&witness), "{}", case.name);
        assert!(
            result
                .iter()
                .all(|row| input.contains(row) || row.0 == case.id.witness_relation())
        );
        assert!(!closure(&[case.id.definition()], &[]).contains(&witness));
        for removed in 0..input.len() {
            // bounded: one of at most three essential premises removed.
            let mut partial = input.clone();
            partial.remove(removed);
            assert!(
                !closure(&[case.id.definition()], &partial).contains(&witness),
                "{} premise {removed}",
                case.name
            );
        }
    }
}

#[test]
fn joins_do_not_confuse_different_subjects_predicates_or_classes() {
    for (id, rows) in [
        (
            C::EqDiff1,
            vec![["a", "owl:sameAs", "b"], ["a", "owl:differentFrom", "c"]],
        ),
        (
            C::PrpIrp,
            vec![
                ["p", "rdf:type", "owl:IrreflexiveProperty"],
                ["a", "p", "b"],
            ],
        ),
        (
            C::PrpAsyp,
            vec![
                ["p", "rdf:type", "owl:AsymmetricProperty"],
                ["a", "p", "b"],
                ["b", "q", "a"],
            ],
        ),
        (
            C::PrpPdw,
            vec![
                ["p", "owl:propertyDisjointWith", "q"],
                ["a", "p", "b"],
                ["c", "q", "b"],
            ],
        ),
        (
            C::CaxDw,
            vec![
                ["A", "owl:disjointWith", "B"],
                ["a", "rdf:type", "A"],
                ["b", "rdf:type", "B"],
            ],
        ),
        (
            C::ClsCom,
            vec![
                ["A", "owl:complementOf", "B"],
                ["a", "rdf:type", "A"],
                ["a", "rdf:type", "C"],
            ],
        ),
        (C::ClsNothing2, vec![["a", "rdf:type", "owl:Class"]]),
    ] {
        // bounded: at most three negative fixture premises.
        let input: Vec<_> = rows.iter().map(fact).collect();
        assert!(
            closure(&[id.definition()], &input)
                .iter()
                .all(|row| row.0 != id.witness_relation())
        );
    }
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Nested {
    id: C,
    program: Program,
}

#[test]
fn identities_and_checked_clauses_survive_nested_transport() {
    for id in C::ALL {
        let value = Nested {
            id: *id,
            program: Program {
                rules: vec![id.definition()],
            },
        };
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
        let mut malformed = value;
        malformed.program.rules[0].head.terms[0] = Term::Variable(VariableSymbol::new("unbound"));
        assert!(
            serde_json::from_str::<Nested>(&serde_json::to_string(&malformed).unwrap()).is_err()
        );
        assert!(
            postcard::from_bytes::<Nested>(&postcard::to_allocvec(&malformed).unwrap()).is_err()
        );
    }
    assert!(serde_json::from_str::<C>("\"unknown\"").is_err());
    assert!(serde_json::from_str::<C>("{\"eq-diff1\":{\"extra\":true}}").is_err());
    for preset in Preset::ALL {
        assert!(
            preset
                .program()
                .rules
                .iter()
                .all(|rule| rule.head.predicate.as_str() == TRIPLE_RELATION)
        );
    }
}
