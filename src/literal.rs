//! Positive literal value-class rules with caller-owned value interpretation.
use crate::rule::Pattern;
use datalog_structural_model::{Atom, Literal, PredicateSymbol, Rule};

/// Binary relation `(literal, value_class)` supplied by a semantic adapter.
///
/// Literal identities stay injective and exact. Class keys identify equal
/// interpreted values under the selected datatype or compatibility contract.
/// They do not replace source RDF terms. Values without an equivalence-class
/// key need separate handling; arbitrary Python equality is not assumed to be
/// a keyable equivalence relation.
pub const LITERAL_VALUE_RELATION: &str = "urn:rdf-reasoning:literal-value-class";

/// Literal clauses whose value-class relation is an explicit caller capability.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum LiteralRuleId {
    /// Substitute exact literal objects that belong to the same value class.
    RdfsValueSubstitution,
}

impl LiteralRuleId {
    /// Every implemented literal value-class clause.
    pub const ALL: &'static [Self] = &[Self::RdfsValueSubstitution];

    /// Construct the checked positive clause over triples and value classes.
    #[must_use]
    pub fn definition(self) -> Rule {
        // bounded: one triple atom and two binary value-class atoms.
        Rule::try_new(
            Pattern(["?s", "?p", "?replacement"]).atom(),
            vec![
                Literal::Positive(Pattern(["?s", "?p", "?original"]).atom()),
                Literal::Positive(Self::value_class(["?original", "?key"])),
                Literal::Positive(Self::value_class(["?replacement", "?key"])),
            ],
        )
        .expect("literal value substitution must be safe Datalog")
    }

    fn value_class(terms: [&str; 2]) -> Atom {
        // bounded: exactly two operands of one semantic relation.
        Atom {
            predicate: PredicateSymbol::new(LITERAL_VALUE_RELATION),
            terms: terms.into_iter().map(Pattern::term).collect(),
        }
    }
}
