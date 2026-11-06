//! Numeric membership consequences and explicit nonmembership witnesses.

use crate::rule::Pattern;
use datalog_structural_model::{Atom, Literal, PredicateSymbol, Rule};

/// Binary `(literal, datatype_iri)` evidence of numeric value-space membership.
///
/// The adapter interprets well-typed literals and enumerates selected datatypes
/// from [`crate::OwlNumericDatatype`], binding their exact `iri()` constants.
/// Supply a tuple exactly when [`crate::OwlNumericValue::is_member_of`] holds.
/// Unknown/unsupported values require an explicit interpretation error, not a
/// guessed membership. Preserve exact literal identity and selected graph scope.
/// `owl:real` and `owl:rational` are general OWL extensions, excluded in OWL-RL
/// selections; this relation does not itself select a complete profile.
pub const OWL_NUMERIC_MEMBER_RELATION: &str = "urn:rdf-reasoning:owl-numeric-member";

/// Binary `(literal, datatype_iri)` evidence of known numeric nonmembership.
///
/// Supply tuples only for successfully interpreted numeric values and selected
/// supported datatypes when `is_member_of` returns false. Absence from the member
/// relation is not evidence. Never insert unsupported or ill-typed literals here.
/// The positive and negative evidence for a pair must not contradict each other.
pub const OWL_NUMERIC_NONMEMBER_RELATION: &str = "urn:rdf-reasoning:owl-numeric-nonmember";

/// Binary `(literal, datatype_iri)` witnesses for asserted incompatible types.
///
/// This is a diagnostic relation, never RDF data or an `owl:Nothing` assertion.
/// A tuple witnesses the `dt-not-type` contradiction, joining a generalized
/// `rdf:type` assertion to explicit nonmembership in the same selected scope.
/// Adapters retain the interpreted value and use
/// [`crate::OwlNumericTypeViolation::check_assertion`] to construct typed reports.
/// A violation makes the selected interpretation inconsistent; callers own
/// reporting/termination policy and must not treat it as an ordinary inference.
pub const OWL_NUMERIC_TYPE_VIOLATION_RELATION: &str =
    "urn:rdf-reasoning:owl-numeric-type-violation";

/// Restricted numeric membership and incompatible-type witness clauses.
///
/// These require explicit value-space evidence, not negation-as-failure.
/// Existing presets are unaffected. The six numeric selectors include general
/// OWL real/rational extensions; an OWL-RL selection must exclude those two.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub enum OwlNumericMembershipRuleId {
    /// Infer a generalized rdf:type assertion from numeric membership evidence.
    #[serde(rename = "numeric/dt-type2")]
    DtType2,
    /// Emit a diagnostic witness for an asserted type with known nonmembership.
    #[serde(rename = "numeric/dt-not-type")]
    DtNotType,
}

impl OwlNumericMembershipRuleId {
    /// Both restricted clauses, separate from numeric identity and preset lists.
    pub const ALL: &'static [Self] = &[Self::DtType2, Self::DtNotType];

    /// Construct a checked positive membership or diagnostic-witness clause.
    #[must_use]
    pub fn definition(self) -> Rule {
        let assertion = Pattern(["?literal", "rdf:type", "?datatype"]).atom();
        // bounded: one membership atom or two assertion/nonmembership atoms.
        let (head, body) = match self {
            Self::DtType2 => (assertion, vec![Self::pair(OWL_NUMERIC_MEMBER_RELATION)]),
            Self::DtNotType => (
                Self::pair(OWL_NUMERIC_TYPE_VIOLATION_RELATION),
                vec![assertion, Self::pair(OWL_NUMERIC_NONMEMBER_RELATION)],
            ),
        };
        Rule::try_new(head, body.into_iter().map(Literal::Positive).collect())
            .expect("numeric membership clauses must be safe Datalog")
    }

    fn pair(relation: &str) -> Atom {
        // bounded: two bound operands of one semantic relation.
        Atom {
            predicate: PredicateSymbol::new(relation),
            terms: ["?literal", "?datatype"]
                .into_iter()
                .map(Pattern::term)
                .collect(),
        }
    }
}
