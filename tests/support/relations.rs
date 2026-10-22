//! Finite fixture interpreter for auxiliary relation assertions.
use datalog_structural_model::{Literal, Rule, Term};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Fact(pub String, pub Vec<String>);

impl Fact {
    pub fn new(relation: &str, terms: &[&str]) -> Self {
        Self(
            relation.to_owned(),
            terms.iter().map(|term| (*term).to_owned()).collect(),
        )
    }
}

pub fn closure(rules: &[Rule], input: &[Fact]) -> BTreeSet<Fact> {
    assert!(input.len() <= 12);
    // bounded: these fixtures admit at most 128 distinct derived facts.
    let mut facts: BTreeSet<_> = input.iter().cloned().collect();
    loop {
        let mut additions = BTreeSet::new();
        for rule in rules {
            let mut bindings = vec![BTreeMap::<String, String>::new()];
            for literal in &rule.body {
                let Literal::Positive(atom) = literal else {
                    panic!("positive fixture clause")
                };
                let mut next = Vec::new();
                for binding in bindings {
                    for fact in &facts {
                        if atom.predicate.as_str() != fact.0 || atom.terms.len() != fact.1.len() {
                            continue;
                        }
                        let mut bound = binding.clone();
                        let matches =
                            atom.terms
                                .iter()
                                .zip(&fact.1)
                                .all(|(term, value)| match term {
                                    Term::Variable(variable) => {
                                        match bound.get(variable.as_str()) {
                                            Some(previous) => previous == value,
                                            None => {
                                                bound.insert(
                                                    variable.as_str().to_owned(),
                                                    value.clone(),
                                                );
                                                true
                                            }
                                        }
                                    }
                                    Term::Constant(constant) => constant.as_str() == value,
                                    _ => panic!("flat fixture term"),
                                });
                        if matches {
                            next.push(bound);
                        }
                    }
                }
                assert!(next.len() <= 4096, "finite fixture join budget");
                bindings = next;
            }
            for binding in bindings {
                additions.insert(Fact(
                    rule.head.predicate.as_str().to_owned(),
                    rule.head
                        .terms
                        .iter()
                        .map(|term| match term {
                            Term::Variable(variable) => binding[variable.as_str()].clone(),
                            Term::Constant(constant) => constant.as_str().to_owned(),
                            _ => panic!("flat fixture head"),
                        })
                        .collect(),
                ));
            }
        }
        let before = facts.len();
        facts.extend(additions);
        assert!(facts.len() <= 128, "finite fixture fact budget");
        if before == facts.len() {
            return facts;
        }
    }
}
