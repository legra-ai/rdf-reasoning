//! Explicit flat schema differences in the owlrl 7.6.2 reference package.

use datalog_structural_model::{BuiltinFilter, ConstantSymbol, Literal, Rule, Term};

use crate::RuleId;

/// Versioned reference clauses, separate from standard rule identities.
///
/// These cover the listed schema differences only, not full owlrl compatibility.
/// Their inequality guards compare exact term identities, never datatype values.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Owlrl762RuleId {
    /// Nonreflexive forward equivalent-class inclusion.
    #[serde(rename = "owlrl-7.6.2/scm-eqc1-forward")]
    ScmEqc1Forward,
    /// Nonreflexive reverse equivalent-class inclusion.
    #[serde(rename = "owlrl-7.6.2/scm-eqc1-reverse")]
    ScmEqc1Reverse,
    /// Nonreflexive forward equivalent-property inclusion.
    #[serde(rename = "owlrl-7.6.2/scm-eqp1-forward")]
    ScmEqp1Forward,
    /// Nonreflexive reverse equivalent-property inclusion.
    #[serde(rename = "owlrl-7.6.2/scm-eqp1-reverse")]
    ScmEqp1Reverse,
    /// Subclass transitivity without inferred reflexive conclusions.
    #[serde(rename = "owlrl-7.6.2/scm-sco")]
    ScmSco,
    /// Subproperty transitivity without inferred reflexive conclusions.
    #[serde(rename = "owlrl-7.6.2/scm-spo")]
    ScmSpo,
    /// Mutual subproperty inclusion without reflexive equivalence conclusions.
    #[serde(rename = "owlrl-7.6.2/scm-eqp2")]
    ScmEqp2,
    /// The reference additionally seeds equivalence for an rdf:Property.
    #[serde(rename = "owlrl-7.6.2/rdf-property-self-equivalent")]
    RdfPropertySelfEquivalent,
}

impl Owlrl762RuleId {
    /// Every versioned flat reference clause implemented here.
    pub const ALL: &'static [Self] = &[
        Self::ScmEqc1Forward,
        Self::ScmEqc1Reverse,
        Self::ScmEqp1Forward,
        Self::ScmEqp1Reverse,
        Self::ScmSco,
        Self::ScmSpo,
        Self::ScmEqp2,
        Self::RdfPropertySelfEquivalent,
    ];

    /// Construct the checked clause with its explicit reference behavior.
    ///
    /// Guards omit redundant self conclusions. This models relational
    /// consequences; it does not specify Python callback/witness ordering.
    #[must_use]
    pub fn definition(self) -> Rule {
        let base = match self {
            Self::ScmEqc1Forward => RuleId::ScmEqc1Forward,
            Self::ScmEqc1Reverse => RuleId::ScmEqc1Reverse,
            Self::ScmEqp1Forward => RuleId::ScmEqp1Forward,
            Self::ScmEqp1Reverse => RuleId::ScmEqp1Reverse,
            Self::ScmSco => RuleId::Rdfs11,
            Self::ScmSpo => RuleId::Rdfs5,
            Self::ScmEqp2 => RuleId::ScmEqp2,
            Self::RdfPropertySelfEquivalent => RuleId::Rdfs6,
        };
        let mut rule = base.definition();
        if self == Self::RdfPropertySelfEquivalent {
            rule.head.terms[1] = Term::Constant(ConstantSymbol::new(
                "http://www.w3.org/2002/07/owl#equivalentProperty",
            ));
        } else {
            rule.body.push(Literal::Filter(BuiltinFilter::Ne(
                rule.head.terms[0].clone(),
                rule.head.terms[2].clone(),
            )));
        }
        Rule::try_new(rule.head, rule.body)
            .expect("versioned reference clauses must be safe Datalog")
    }
}
