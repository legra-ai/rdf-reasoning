//! RDF 1.2 predicate/resource typing over trusted recursive occurrence premises.
use crate::{TRIPLE_RELATION, rule::Pattern};
use datalog_structural_model::{Atom, Literal, PredicateSymbol, Rule};

/// Auxiliary relation `(exact_term_key)` for a recursively appearing RDF term.
///
/// Include subject, predicate and object terms and recursively nested components,
/// preserving literal, blank-node and triple-term identity. Literal metadata such
/// as its internal datatype IRI is not another appearing term merely by being
/// metadata. The caller selects an immutable source or target-context scope.
/// In target context, admit only terms whose denotation is established. Atomic
/// components remain eligible after literal validation, but a whole triple term
/// needs source evidence or validated construction with its property premise.
/// Otherwise defer that whole-term premise; do not invent Resource membership.
/// Preserve already established Resource facts and continue recursive traversal.
/// Keep root/path/role/snapshot evidence separately; these premises are not RDF.
pub const RDF12_APPEARING_TERM_RELATION: &str = "urn:rdf-reasoning:rdf12-appearing-term";

/// Auxiliary relation `(subject, predicate, object)` for an appearing triple.
///
/// Include selected source assertions, source-derived triples and triple terms
/// recursively appearing in them. A nested occurrence is not an asserted triple
/// premise: never project this relation into user RDF or ordinary triple joins.
/// Target-context triples must not bind this relation for predicate typing.
pub const RDF12_APPEARING_TRIPLE_RELATION: &str = "urn:rdf-reasoning:rdf12-appearing-triple";

/// Exact RDF 1.2 occurrence rules, independent of restricted legacy presets.
///
/// Source processing admits both rules. Appendix A target-context processing
/// admits only Rdfs4, with blanks standardized apart from source while preserving
/// repeated/nested target identity. Those blanks remain existential target
/// variables, not fixed IRIs or new asserted source properties. Target evidence
/// must not contaminate source consistency or assertion provenance.
/// A target triple term's syntactic occurrence alone does not establish its
/// denotation: the triple interpretation function requires a property in its
/// middle slot. Adapters must enforce this before binding whole-term premises.
///
/// Adapters own bounded async occurrence traversal over the selected authoritative
/// snapshot, including reloading evicted asserted and inferred facts. Cache misses
/// are not exhausted relations. Fail explicitly on read/cancellation/budget or
/// unsupported-term errors; do not silently skip nested terms or assert their
/// contents. Flat Datalog syntax admission does not certify this relation binding.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Rdf12OccurrenceRuleId {
    /// An appearing source triple's predicate is a property, even inside a term.
    #[serde(rename = "rdfD2")]
    RdfD2,
    /// An admitted, denoting appearing term is a resource in the selected context.
    #[serde(rename = "rdfs4")]
    Rdfs4,
}

impl Rdf12OccurrenceRuleId {
    /// Both source rules; target context selects only [`Self::Rdfs4`].
    pub const ALL: &'static [Self] = &[Self::RdfD2, Self::Rdfs4];

    /// Exact standard clause identity, also used by Serde.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::RdfD2 => "rdfD2",
            Self::Rdfs4 => "rdfs4",
        }
    }

    /// Construct a checked one-premise property/resource typing clause.
    ///
    /// This supplies a definition, not a traversal engine, target matcher or
    /// complete closure schedule. Preserve semantic occurrence evidence instead
    /// of fabricating an asserted RDF witness for a quoted or target occurrence.
    #[must_use]
    pub fn definition(self) -> Rule {
        let (head, relation, terms): (_, _, &[&str]) = match self {
            Self::RdfD2 => (
                ["?predicate", "rdf:type", "rdf:Property"],
                RDF12_APPEARING_TRIPLE_RELATION,
                &["?subject", "?predicate", "?object"],
            ),
            Self::Rdfs4 => (
                ["?term", "rdf:type", "rdfs:Resource"],
                RDF12_APPEARING_TERM_RELATION,
                &["?term"],
            ),
        };
        // bounded: one head and one premise, at most three terms each.
        let premise = Atom {
            predicate: PredicateSymbol::new(relation),
            terms: terms.iter().map(|term| Pattern::term(term)).collect(),
        };
        let head = Atom {
            predicate: PredicateSymbol::new(TRIPLE_RELATION),
            terms: head.into_iter().map(Pattern::term).collect(),
        };
        Rule::try_new(head, vec![Literal::Positive(premise)])
            .expect("RDF 1.2 occurrence definitions must be checked safe Datalog")
    }
}
