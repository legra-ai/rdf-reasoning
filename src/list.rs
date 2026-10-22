//! Explicit auxiliary list membership and positive OWL list consequences.
use datalog_structural_model::{Atom, Literal, PredicateSymbol, Rule};

use crate::rule::Pattern;

/// Two-place relation `(list_cell, member)` for reachable RDF list members.
///
/// Its least fixed point follows zero or more `rdf:rest` edges then one
/// `rdf:first` edge. It does not validate list structure or require termination
/// at `rdf:nil`; a caller may separately reject malformed lists.
pub const RDF_LIST_MEMBER_RELATION: &str = "urn:rdf-reasoning:list-member";

/// Positive list clauses, separate from the flat triple-only catalogue.
///
/// The five OWL consequences require the two membership clauses or an adapter
/// providing their exact relation. This does not include intersection
/// recognition, property chains, keys or the complete OWL list semantics.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum ListRuleId {
    /// Each first edge supplies a member of its own cell.
    MemberFirst,
    /// Members of a rest successor are members of the preceding cell.
    MemberRest,
    /// An intersection instance belongs to each listed class.
    ClsInt2,
    /// An instance of any listed class belongs to their union.
    ClsUni,
    /// Each enumerated individual belongs to the enumeration class.
    ClsOo,
    /// An intersection is a subclass of each listed class.
    ScmInt,
    /// Each listed class is a subclass of their union.
    ScmUni,
}

impl ListRuleId {
    /// All auxiliary clauses and their five positive OWL consumers.
    pub const ALL: &'static [Self] = &[
        Self::MemberFirst,
        Self::MemberRest,
        Self::ClsInt2,
        Self::ClsUni,
        Self::ClsOo,
        Self::ScmInt,
        Self::ScmUni,
    ];

    /// Construct a checked Datalog clause with explicit relation identities.
    #[must_use]
    pub fn definition(self) -> Rule {
        // bounded: at most three body atoms in each fixed catalogue clause.
        let (head, body) = match self {
            Self::MemberFirst => (
                Self::member(["?l", "?m"]),
                vec![Pattern(["?l", "rdf:first", "?m"]).atom()],
            ),
            Self::MemberRest => (
                Self::member(["?l", "?m"]),
                vec![
                    Pattern(["?l", "rdf:rest", "?tail"]).atom(),
                    Self::member(["?tail", "?m"]),
                ],
            ),
            Self::ClsInt2 => (
                Pattern(["?x", "rdf:type", "?m"]).atom(),
                vec![
                    Pattern(["?c", "owl:intersectionOf", "?l"]).atom(),
                    Self::member(["?l", "?m"]),
                    Pattern(["?x", "rdf:type", "?c"]).atom(),
                ],
            ),
            Self::ClsUni => (
                Pattern(["?x", "rdf:type", "?c"]).atom(),
                vec![
                    Pattern(["?c", "owl:unionOf", "?l"]).atom(),
                    Self::member(["?l", "?m"]),
                    Pattern(["?x", "rdf:type", "?m"]).atom(),
                ],
            ),
            Self::ClsOo => (
                Pattern(["?x", "rdf:type", "?c"]).atom(),
                vec![
                    Pattern(["?c", "owl:oneOf", "?l"]).atom(),
                    Self::member(["?l", "?x"]),
                ],
            ),
            Self::ScmInt => (
                Pattern(["?c", "rdfs:subClassOf", "?m"]).atom(),
                vec![
                    Pattern(["?c", "owl:intersectionOf", "?l"]).atom(),
                    Self::member(["?l", "?m"]),
                ],
            ),
            Self::ScmUni => (
                Pattern(["?m", "rdfs:subClassOf", "?c"]).atom(),
                vec![
                    Pattern(["?c", "owl:unionOf", "?l"]).atom(),
                    Self::member(["?l", "?m"]),
                ],
            ),
        };
        Rule::try_new(head, body.into_iter().map(Literal::Positive).collect())
            .expect("the positive list catalogue must be safe Datalog")
    }

    fn member(terms: [&str; 2]) -> Atom {
        // bounded: exactly two terms in the list membership relation.
        Atom {
            predicate: PredicateSymbol::new(RDF_LIST_MEMBER_RELATION),
            terms: terms.into_iter().map(Pattern::term).collect(),
        }
    }
}
