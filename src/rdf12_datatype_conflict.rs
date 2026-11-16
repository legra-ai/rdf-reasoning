//! Positive default-map datatype conflict witnesses, separate from RDF output.
use crate::{Rdf12Datatype, TRIPLE_RELATION, rule::Pattern};
use datalog_structural_model::{Atom, ConstantSymbol, Literal, PredicateSymbol, Rule, Term};

/// Three disjoint mandatory default datatype pairs from RDF 1.2 value semantics.
///
/// These are implementation diagnostic IDs, not named W3C entailment rules.
/// Evaluate against the selected source closure, including scoped nonempty-class
/// witnesses, before adding target-context premises. Preserve both exact type
/// premises, this ID and source scope as evidence. A witness is non-RDF diagnostic
/// data, never an asserted triple or a fabricated negative membership assertion.
///
/// Execution must probe persisted asserted and derived memberships asynchronously
/// when needed; cache absence does not establish absence of a premise. Read,
/// cancellation and resource errors must not become a consistency verdict.
/// Existing presets do not select these diagnostics; positive Datalog admission
/// alone does not certify inconsistency reporting or complete consistency checks.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Rdf12DatatypeConflictId {
    /// String values and language-tagged pairs cannot share a member.
    StringLangString,
    /// String values and directional language-tagged triples cannot share a member.
    StringDirLangString,
    /// Language-tagged pairs and directional triples cannot share a member.
    LangStringDirLangString,
}

impl Rdf12DatatypeConflictId {
    /// All three canonical unordered pairs; arbitrary encountered types are excluded.
    pub const ALL: &'static [Self] = &[
        Self::StringLangString,
        Self::StringDirLangString,
        Self::LangStringDirLangString,
    ];

    /// Stable implementation diagnostic name, also used by Serde.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::StringLangString => "string-lang-string",
            Self::StringDirLangString => "string-dir-lang-string",
            Self::LangStringDirLangString => "lang-string-dir-lang-string",
        }
    }

    /// The two disjoint recognized datatype classes in canonical order.
    #[must_use]
    pub const fn datatypes(self) -> [Rdf12Datatype; 2] {
        match self {
            Self::StringLangString => [Rdf12Datatype::String, Rdf12Datatype::LangString],
            Self::StringDirLangString => [Rdf12Datatype::String, Rdf12Datatype::DirLangString],
            Self::LangStringDirLangString => {
                [Rdf12Datatype::LangString, Rdf12Datatype::DirLangString]
            }
        }
    }

    /// Unary diagnostic relation `(exact_member_key)`, never projected into user RDF.
    #[must_use]
    pub const fn witness_relation(self) -> &'static str {
        match self {
            Self::StringLangString => "urn:rdf-reasoning:rdf12-violation:string-lang-string",
            Self::StringDirLangString => "urn:rdf-reasoning:rdf12-violation:string-dir-lang-string",
            Self::LangStringDirLangString => {
                "urn:rdf-reasoning:rdf12-violation:lang-string-dir-lang-string"
            }
        }
    }

    /// Checked positive two-premise clause reporting one shared member key.
    ///
    /// Both class IRIs are constants. Graph datatype declarations do not extend
    /// recognition, distinct member keys are not guessed equal, and absent
    /// premises never establish nonmembership. This performs no lexical mapping.
    #[must_use]
    pub fn definition(self) -> Rule {
        // bounded: one projected term and two fixed three-term premises.
        let head = Atom {
            predicate: PredicateSymbol::new(self.witness_relation()),
            terms: vec![Pattern::term("?member")],
        };
        let body = self
            .datatypes()
            .into_iter()
            .map(|datatype| {
                Literal::Positive(Atom {
                    predicate: PredicateSymbol::new(TRIPLE_RELATION),
                    terms: vec![
                        Pattern::term("?member"),
                        Pattern::term("rdf:type"),
                        Term::Constant(ConstantSymbol::new(datatype.iri())),
                    ],
                })
            })
            .collect();
        Rule::try_new(head, body).expect("default datatype conflicts must be checked safe Datalog")
    }
}
