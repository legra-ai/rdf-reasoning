//! Independently specified tiny rule fixtures; no production graph evaluator.

use datalog_structural_model::{Atom, Literal, Rule, Term};
use rdf_reasoning::{RuleId, TRIPLE_RELATION};
use std::collections::BTreeMap;

#[test]
fn each_rule_has_its_specified_consequence() {
    let fixtures: &[(RuleId, &[&str], &str)] = &[
        (RuleId::Rdf1, &["a p b"], "p rdf:type rdf:Property"),
        (RuleId::Rdfs2, &["p rdfs:domain C", "a p b"], "a rdf:type C"),
        (RuleId::Rdfs3, &["p rdfs:range C", "a p b"], "b rdf:type C"),
        (RuleId::Rdfs4a, &["a p b"], "a rdf:type rdfs:Resource"),
        (
            RuleId::Rdfs4b,
            &["a p literal:value"],
            "literal:value rdf:type rdfs:Resource",
        ),
        (
            RuleId::Rdfs5,
            &["p rdfs:subPropertyOf q", "q rdfs:subPropertyOf r"],
            "p rdfs:subPropertyOf r",
        ),
        (
            RuleId::Rdfs6,
            &["p rdf:type rdf:Property"],
            "p rdfs:subPropertyOf p",
        ),
        (RuleId::Rdfs7, &["p rdfs:subPropertyOf q", "a p b"], "a q b"),
        (
            RuleId::Rdfs8,
            &["C rdf:type rdfs:Class"],
            "C rdfs:subClassOf rdfs:Resource",
        ),
        (
            RuleId::Rdfs9,
            &["a rdf:type C", "C rdfs:subClassOf D"],
            "a rdf:type D",
        ),
        (
            RuleId::Rdfs10,
            &["C rdf:type rdfs:Class"],
            "C rdfs:subClassOf C",
        ),
        (
            RuleId::Rdfs11,
            &["C rdfs:subClassOf D", "D rdfs:subClassOf E"],
            "C rdfs:subClassOf E",
        ),
        (
            RuleId::Rdfs12,
            &["p rdf:type rdfs:ContainerMembershipProperty"],
            "p rdfs:subPropertyOf rdfs:member",
        ),
        (
            RuleId::Rdfs13,
            &["D rdf:type rdfs:Datatype"],
            "D rdfs:subClassOf rdfs:Literal",
        ),
        (
            RuleId::PrpSymp,
            &["p rdf:type owl:SymmetricProperty", "a p b"],
            "b p a",
        ),
        (
            RuleId::PrpTrp,
            &["p rdf:type owl:TransitiveProperty", "a p b", "b p c"],
            "a p c",
        ),
        (RuleId::PrpInv1, &["p owl:inverseOf q", "a p b"], "b q a"),
        (RuleId::PrpInv2, &["p owl:inverseOf q", "a q b"], "b p a"),
    ];
    for (id, premises, conclusion) in fixtures {
        let rule = id.definition();
        let actual = Fixture::apply(&rule, premises);
        assert!(
            actual.contains(&Fixture::triple(conclusion)),
            "{id:?}: {actual:?}"
        );
        assert!(
            Fixture::apply(&rule, &[]).is_empty(),
            "{id:?} inferred without premises"
        );
    }
}

#[test]
fn joins_and_constants_are_not_wildcards() {
    assert!(
        Fixture::apply(
            &RuleId::Rdfs9.definition(),
            &["a rdf:type C", "Other rdfs:subClassOf D"]
        )
        .is_empty()
    );
    assert!(
        Fixture::apply(
            &RuleId::PrpTrp.definition(),
            &["p rdf:type owl:TransitiveProperty", "a p b", "b q c"]
        )
        .is_empty()
    );
    assert!(
        Fixture::apply(
            &RuleId::PrpSymp.definition(),
            &["p rdf:type owl:FunctionalProperty", "a p b"]
        )
        .is_empty()
    );
}

struct Fixture;

impl Fixture {
    fn triple(text: &str) -> [String; 3] {
        let mut terms = text.split_whitespace().map(Self::expand);
        let triple = std::array::from_fn(|_| terms.next().expect("fixture term"));
        assert!(terms.next().is_none());
        triple
    }

    fn expand(term: &str) -> String {
        for (prefix, namespace) in [
            ("rdf:", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
            ("rdfs:", "http://www.w3.org/2000/01/rdf-schema#"),
            ("owl:", "http://www.w3.org/2002/07/owl#"),
        ] {
            if let Some(local) = term.strip_prefix(prefix) {
                return format!("{namespace}{local}");
            }
        }
        term.to_owned()
    }

    fn apply(rule: &Rule, input: &[&str]) -> Vec<[String; 3]> {
        assert!(input.len() <= 3);
        // bounded: at most three fixture facts; at most 3^3 candidate joins.
        let facts: Vec<_> = input.iter().map(|text| Self::triple(text)).collect();
        let mut bindings = vec![BTreeMap::new()];
        for literal in &rule.body {
            let Literal::Positive(atom) = literal else {
                panic!("non-positive core rule")
            };
            let mut next = Vec::new();
            for binding in bindings {
                for fact in &facts {
                    if let Some(bound) = Self::unify(atom, fact, &binding) {
                        next.push(bound);
                    }
                }
            }
            bindings = next;
        }
        bindings
            .iter()
            .map(|binding| {
                std::array::from_fn(|index| match &rule.head.terms[index] {
                    Term::Variable(variable) => binding[variable.as_str()].clone(),
                    Term::Constant(constant) => constant.as_str().to_owned(),
                    _ => panic!("non-flat core rule"),
                })
            })
            .collect()
    }

    fn unify(
        atom: &Atom,
        fact: &[String; 3],
        binding: &BTreeMap<String, String>,
    ) -> Option<BTreeMap<String, String>> {
        assert_eq!(atom.predicate.as_str(), TRIPLE_RELATION);
        assert_eq!(atom.terms.len(), 3);
        // bounded: each fixture rule has at most five variables.
        let mut bound = binding.clone();
        for (term, value) in atom.terms.iter().zip(fact) {
            match term {
                Term::Variable(variable) => {
                    if let Some(previous) =
                        bound.insert(variable.as_str().to_owned(), value.clone())
                        && previous != *value
                    {
                        return None;
                    }
                }
                Term::Constant(constant) if constant.as_str() == value => {}
                Term::Constant(_) => return None,
                _ => panic!("non-flat core rule"),
            }
        }
        Some(bound)
    }
}
