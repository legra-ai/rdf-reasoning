//! Restricted OWL numeric value identity clauses over an explicit adapter relation.

use crate::rule::Pattern;
use datalog_structural_model::{Atom, BuiltinFilter, Literal, PredicateSymbol, Rule};

/// Binary relation `(literal, numeric_value_key)` for interpreted OWL numeric values.
///
/// Supply exactly one key per in-scope, well-typed numeric literal. Key identity
/// must agree exactly with [`crate::OwlNumericValue`] identity: decimal/float/double
/// spaces and signed zeros are distinct, with one NaN per floating space. Keys
/// are separate from exact source literal identities and must be collision-free;
/// hash digests alone and lexical spellings are not value identities. In
/// particular, Serde bytes are not promised to be a canonical identity encoding.
///
/// The adapter validates lexical forms before assigning values and supplies all
/// numeric literals in its explicitly selected scope, including literals in
/// generalized tuples. It must reject unsupported interpretation/capabilities
/// instead of assigning a shared unknown key or treating term inequality as
/// value inequality. This relation does not interpret nonnumeric literals or
/// implement complete datatype semantics. It is deliberately separate from
/// [`crate::LITERAL_VALUE_RELATION`], whose compatibility contract may differ.
pub const OWL_NUMERIC_VALUE_RELATION: &str = "urn:rdf-reasoning:owl-numeric-value";

/// OWL `dt-eq` and `dt-diff` restricted to interpreted decimal/float/double values.
///
/// Select these independently of existing presets. Adapters supply the complete
/// selected numeric relation and execute exact key joins/inequality plus
/// generalized triple heads. This is not a full datatype or OWL-RL profile.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum OwlNumericRuleId {
    /// Emit owl:sameAs for all ordered literal pairs with identical numeric values.
    #[serde(rename = "numeric/dt-eq")]
    DtEq,
    /// Emit owl:differentFrom for all ordered pairs with distinct numeric values.
    #[serde(rename = "numeric/dt-diff")]
    DtDiff,
}

impl OwlNumericRuleId {
    /// Both implemented clauses, each with the same restricted numeric domain.
    pub const ALL: &'static [Self] = &[Self::DtEq, Self::DtDiff];

    /// Construct a checked Datalog clause producing generalized RDF triples.
    ///
    /// `DtEq` joins one shared numeric key, including literal self-pairs.
    /// `DtDiff` binds both numeric keys before applying structural `Ne` to those
    /// keys. It never compares source literal spellings or invokes IEEE equality.
    #[must_use]
    pub fn definition(self) -> Rule {
        let predicate = match self {
            Self::DtEq => "owl:sameAs",
            Self::DtDiff => "owl:differentFrom",
        };
        let right_key = match self {
            Self::DtEq => "?left_key",
            Self::DtDiff => "?right_key",
        };
        // bounded: exactly two binary value atoms and at most one key filter.
        let mut body = vec![
            Literal::Positive(Self::value(["?left", "?left_key"])),
            Literal::Positive(Self::value(["?right", right_key])),
        ];
        if self == Self::DtDiff {
            body.push(Literal::Filter(BuiltinFilter::Ne(
                Pattern::term("?left_key"),
                Pattern::term("?right_key"),
            )));
        }
        Rule::try_new(Pattern(["?left", predicate, "?right"]).atom(), body)
            .expect("numeric value identity clauses must be safe Datalog")
    }

    fn value(terms: [&str; 2]) -> Atom {
        // bounded: exactly two terms in one numeric value relation atom.
        Atom {
            predicate: PredicateSymbol::new(OWL_NUMERIC_VALUE_RELATION),
            terms: terms.into_iter().map(Pattern::term).collect(),
        }
    }
}
