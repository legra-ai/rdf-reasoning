//! Explicit Grdfs14 construction from RDF 1.2 Semantics CRD 2026-09-24.
use crate::{TRIPLE_RELATION, rule::Pattern};
use datalog_structural_model::{Atom, Literal, PredicateSymbol, Rule, Term};

/// Generalized proposition construction, separate from ordinary closure presets.
///
/// An executor must explicitly admit three-slot triple terms and a finite
/// construction phase over an immutable, caller-selected snapshot. It must not
/// feed newly constructed terms recursively into unrestricted construction.
/// Choosing adequate typing and target/substitution context remains a separate
/// entailment-controller obligation; this clause does not certify a complete
/// schedule for the postulated Appendix A procedure.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Rdf12PropositionRuleId {
    /// Construct a proposition term from a property and two resources.
    #[serde(rename = "grdfs14")]
    Grdfs14,
}

impl Rdf12PropositionRuleId {
    /// The one constructor clause; this excludes occurrence substitution rdfs14.
    pub const ALL: &'static [Self] = &[Self::Grdfs14];

    /// Stable clause identifier.
    #[must_use]
    pub const fn name(self) -> &'static str {
        "grdfs14"
    }

    /// Checked three-premise Grdfs14 with a canonical three-slot triple-term head.
    ///
    /// The nested atom uses `TRIPLE_RELATION` with `(subject, predicate, object)`
    /// terms; its relation symbol is not the RDF predicate. Preserve generalized
    /// bindings, including literal predicate keys and nested terms. Quoted content
    /// is not asserted. Evidence retains this ID, all three exact premise tuples
    /// and the selected phase/snapshot. Check substituted nesting and resource
    /// limits explicitly: template admission alone cannot certify actual output.
    #[must_use]
    pub fn definition(self) -> Rule {
        let subject = Pattern::term("?subject");
        let predicate = Pattern::term("?predicate");
        let object = Pattern::term("?object");
        // bounded: one head and three body atoms, each containing three terms.
        let quoted = Term::TripleTerm(Box::new(Atom {
            predicate: PredicateSymbol::new(TRIPLE_RELATION),
            terms: vec![subject.clone(), predicate.clone(), object.clone()],
        }));
        let head = Self::membership(quoted, "rdfs:Proposition");
        let body = vec![
            Literal::Positive(Self::membership(predicate, "rdf:Property")),
            Literal::Positive(Self::membership(subject, "rdfs:Resource")),
            Literal::Positive(Self::membership(object, "rdfs:Resource")),
        ];
        Rule::try_new(head, body).expect("Grdfs14 must be a checked safe constructor clause")
    }

    fn membership(term: Term, class: &str) -> Atom {
        Atom {
            predicate: PredicateSymbol::new(TRIPLE_RELATION),
            // bounded: the three fixed RDF relation slots.
            terms: vec![term, Pattern::term("rdf:type"), Pattern::term(class)],
        }
    }
}
