use datalog_structural_model::{Atom, BuiltinFilter, Literal, Rule, Term};
use rdf_reasoning::TRIPLE_RELATION;
use std::collections::BTreeMap;

pub(crate) struct Fixture;

impl Fixture {
    pub(crate) fn triple(text: &str) -> [String; 3] {
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

    pub(crate) fn apply(rule: &Rule, input: &[&str]) -> Vec<[String; 3]> {
        assert!(input.len() <= 5);
        // bounded: at most five fixture facts; at most 5^5 candidate joins.
        let facts: Vec<_> = input.iter().map(|text| Self::triple(text)).collect();
        let mut bindings = vec![BTreeMap::new()];
        for literal in &rule.body {
            if let Literal::Filter(BuiltinFilter::Ne(left, right)) = literal {
                bindings
                    .retain(|binding| Self::value(left, binding) != Self::value(right, binding));
                continue;
            }
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

    fn value<'a>(term: &'a Term, binding: &'a BTreeMap<String, String>) -> &'a str {
        match term {
            Term::Variable(variable) => &binding[variable.as_str()],
            Term::Constant(constant) => constant.as_str(),
            _ => panic!("non-flat fixture term"),
        }
    }

    fn unify(
        atom: &Atom,
        fact: &[String; 3],
        binding: &BTreeMap<String, String>,
    ) -> Option<BTreeMap<String, String>> {
        assert_eq!(atom.predicate.as_str(), TRIPLE_RELATION);
        assert_eq!(atom.terms.len(), 3);
        // bounded: each fixture rule has at most seven variables.
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
