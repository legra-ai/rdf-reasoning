//! Tiny independent recursive occurrence fixture, never a production graph scan.
use datalog_structural_model::{Atom, PredicateSymbol, Term};
use rdf_reasoning::{RDF12_APPEARING_TERM_RELATION, RDF12_APPEARING_TRIPLE_RELATION};

pub fn occurrences(root: &Atom) -> Vec<Atom> {
    // bounded: one fixture root, depth at most four and at most twelve events.
    let mut events = Vec::new();
    visit(root, 0, &mut events);
    events
}

fn visit(triple: &Atom, depth: usize, events: &mut Vec<Atom>) {
    assert!(depth <= 4, "fixed fixture depth ceiling");
    assert_eq!(triple.terms.len(), 3);
    events.push(Atom {
        predicate: PredicateSymbol::new(RDF12_APPEARING_TRIPLE_RELATION),
        terms: triple.terms.clone(),
    });
    for term in &triple.terms {
        events.push(Atom {
            predicate: PredicateSymbol::new(RDF12_APPEARING_TERM_RELATION),
            terms: vec![term.clone()],
        });
        if let Term::TripleTerm(nested) = term {
            visit(nested, depth + 1, events);
        }
    }
    assert!(events.len() <= 12, "fixed fixture event ceiling");
}
