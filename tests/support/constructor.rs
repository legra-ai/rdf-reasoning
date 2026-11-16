//! Bounded, single-phase interpreter for an independently selected fixture snapshot.
use datalog_structural_model::{Atom, Literal, ProgramValidationError, Rule, Term};
use std::collections::BTreeMap;

pub fn apply_once(rule: &Rule, snapshot: &[Atom]) -> Result<Vec<Atom>, ProgramValidationError> {
    assert!(snapshot.len() <= 12, "fixed fixture snapshot ceiling");
    // bounded: at most twelve input facts and three body joins in these fixtures.
    let mut bindings = vec![BTreeMap::new()];
    for premise in &rule.body {
        let Literal::Positive(atom) = premise else {
            panic!("positive fixture premises only");
        };
        let mut next = Vec::new();
        for binding in &bindings {
            for fact in snapshot {
                if atom.predicate != fact.predicate || atom.terms.len() != fact.terms.len() {
                    continue;
                }
                let mut bound = binding.clone();
                if atom
                    .terms
                    .iter()
                    .zip(&fact.terms)
                    .all(|(pattern, value)| match pattern {
                        Term::Variable(variable) => match bound.get(variable.as_str()) {
                            Some(previous) => previous == value,
                            None => {
                                bound.insert(variable.as_str().to_owned(), value.clone());
                                true
                            }
                        },
                        Term::Constant(_) => pattern == value,
                        _ => panic!("flat fixture body patterns only"),
                    })
                {
                    next.push(bound);
                }
                assert!(next.len() <= 1728, "fixed fixture join ceiling");
            }
        }
        bindings = next;
    }
    bindings
        .iter()
        .map(|binding| {
            let head = instantiate_atom(&rule.head, binding);
            // Checked ground construction must reject excess nesting after substitution.
            Rule::try_new(head, vec![]).map(|checked| checked.head)
        })
        .collect()
}

fn instantiate_atom(atom: &Atom, binding: &BTreeMap<String, Term>) -> Atom {
    Atom {
        predicate: atom.predicate.clone(),
        // bounded: exactly three slots per fixture atom, model-bounded nesting.
        terms: atom
            .terms
            .iter()
            .map(|term| match term {
                Term::Variable(variable) => binding[variable.as_str()].clone(),
                Term::Constant(_) => term.clone(),
                Term::TripleTerm(inner) => {
                    Term::TripleTerm(Box::new(instantiate_atom(inner, binding)))
                }
                _ => panic!("unsupported fixture head term"),
            })
            .collect(),
    }
}
