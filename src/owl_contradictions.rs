//! Flat OWL 2 RL/RDF contradiction witnesses, separate from RDF consequences.
use crate::rule::Pattern;
use datalog_structural_model::{Atom, Literal, PredicateSymbol, Rule};

/// Seven flat contradiction clauses from OWL 2 RL/RDF Second Edition (2012).
///
/// Heads are diagnostic relations, never user RDF. Positive Datalog support is
/// insufficient to certify inconsistency detection: an executor must explicitly
/// opt into the witness contract and reporting policy. Existing presets do not
/// select these clauses. This catalogue is not a generic denial executor.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum OwlContradictionRuleId {
    /// Equality and difference for the same ordered pair: witness `(x, y)`.
    EqDiff1,
    /// Irreflexive property used on itself: witness `(p, x)`.
    PrpIrp,
    /// Asymmetric property used in both directions: witness `(p, x, y)`.
    PrpAsyp,
    /// Disjoint properties share an assertion: witness `(p1, p2, x, y)`.
    PrpPdw,
    /// Disjoint classes share an instance: witness `(c1, c2, x)`.
    CaxDw,
    /// Complement classes share an instance: witness `(c1, c2, x)`.
    ClsCom,
    /// An instance belongs to Nothing: witness `(x)`.
    ClsNothing2,
}

impl OwlContradictionRuleId {
    /// All seven flat diagnostic clauses, independent of ordinary presets.
    pub const ALL: &'static [Self] = &[
        Self::EqDiff1,
        Self::PrpIrp,
        Self::PrpAsyp,
        Self::PrpPdw,
        Self::CaxDw,
        Self::ClsCom,
        Self::ClsNothing2,
    ];

    /// Exact standard clause identity, also used by Serde.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::EqDiff1 => "eq-diff1",
            Self::PrpIrp => "prp-irp",
            Self::PrpAsyp => "prp-asyp",
            Self::PrpPdw => "prp-pdw",
            Self::CaxDw => "cax-dw",
            Self::ClsCom => "cls-com",
            Self::ClsNothing2 => "cls-nothing2",
        }
    }

    /// Stable non-RDF diagnostic relation with the variant's ordered projection.
    ///
    /// Preserve exact bound RDF identities and selected source scope. Equal
    /// witness tuples have ordinary relation-set identity; multiplicity, order
    /// and provenance are not supplied by this catalogue. Consumers must never
    /// project these tuples into RDF or silently discard unsupported diagnostics.
    #[must_use]
    pub const fn witness_relation(self) -> &'static str {
        match self {
            Self::EqDiff1 => "urn:rdf-reasoning:owl-violation:eq-diff1",
            Self::PrpIrp => "urn:rdf-reasoning:owl-violation:prp-irp",
            Self::PrpAsyp => "urn:rdf-reasoning:owl-violation:prp-asyp",
            Self::PrpPdw => "urn:rdf-reasoning:owl-violation:prp-pdw",
            Self::CaxDw => "urn:rdf-reasoning:owl-violation:cax-dw",
            Self::ClsCom => "urn:rdf-reasoning:owl-violation:cls-com",
            Self::ClsNothing2 => "urn:rdf-reasoning:owl-violation:cls-nothing2",
        }
    }

    /// Construct a checked positive clause producing a contradiction witness.
    ///
    /// A matching body establishes the identified inconsistency. This is not
    /// closed-world negation; no missing premise implies a violation. The caller
    /// owns bounded execution, evidence retention, reporting and termination.
    #[must_use]
    pub fn definition(self) -> Rule {
        let (projection, body): (&[&str], &[[&str; 3]]) = match self {
            Self::EqDiff1 => (
                &["?x", "?y"],
                &[
                    ["?x", "owl:sameAs", "?y"],
                    ["?x", "owl:differentFrom", "?y"],
                ],
            ),
            Self::PrpIrp => (
                &["?p", "?x"],
                &[
                    ["?p", "rdf:type", "owl:IrreflexiveProperty"],
                    ["?x", "?p", "?x"],
                ],
            ),
            Self::PrpAsyp => (
                &["?p", "?x", "?y"],
                &[
                    ["?p", "rdf:type", "owl:AsymmetricProperty"],
                    ["?x", "?p", "?y"],
                    ["?y", "?p", "?x"],
                ],
            ),
            Self::PrpPdw => (
                &["?p1", "?p2", "?x", "?y"],
                &[
                    ["?p1", "owl:propertyDisjointWith", "?p2"],
                    ["?x", "?p1", "?y"],
                    ["?x", "?p2", "?y"],
                ],
            ),
            Self::CaxDw => (
                &["?c1", "?c2", "?x"],
                &[
                    ["?c1", "owl:disjointWith", "?c2"],
                    ["?x", "rdf:type", "?c1"],
                    ["?x", "rdf:type", "?c2"],
                ],
            ),
            Self::ClsCom => (
                &["?c1", "?c2", "?x"],
                &[
                    ["?c1", "owl:complementOf", "?c2"],
                    ["?x", "rdf:type", "?c1"],
                    ["?x", "rdf:type", "?c2"],
                ],
            ),
            Self::ClsNothing2 => (&["?x"], &[["?x", "rdf:type", "owl:Nothing"]]),
        };
        // bounded: at most four projected terms and three fixed body atoms.
        let head = Atom {
            predicate: PredicateSymbol::new(self.witness_relation()),
            terms: projection.iter().map(|term| Pattern::term(term)).collect(),
        };
        Rule::try_new(
            head,
            body.iter()
                .map(|row| Literal::Positive(Pattern(*row).atom()))
                .collect(),
        )
        .expect("flat OWL contradiction clauses must be checked safe Datalog")
    }
}
