//! Finite, independently authored relational rule catalogue.

mod owl;

use datalog_structural_model::{
    Atom, ConstantSymbol, Literal, PredicateSymbol, Rule, Term, VariableSymbol,
};

/// The three-place relation used by every rule: `(subject, predicate, object)`.
///
/// This includes generalized intermediate tuples. Adapters preserve term identity
/// and must not discard tuples solely because a subject is a literal.
pub const TRIPLE_RELATION: &str = "urn:rdf-reasoning:triple";

/// Exact identity of a portable rule; a preset is a finite selection of these.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum RuleId {
    /// A predicate occurring in a triple is an RDF property.
    Rdf1,
    /// Property domain implies subject membership.
    Rdfs2,
    /// Property range implies object membership.
    Rdfs3,
    /// Triple subjects are resources.
    Rdfs4a,
    /// Triple objects are resources, including generalized literal subjects.
    Rdfs4b,
    /// Subproperty transitivity.
    Rdfs5,
    /// A declared property is its own subproperty.
    Rdfs6,
    /// Subproperty inheritance of assertions.
    Rdfs7,
    /// Declared classes are subclasses of Resource.
    Rdfs8,
    /// Subclass inheritance of membership.
    Rdfs9,
    /// A declared class is its own subclass.
    Rdfs10,
    /// Subclass transitivity.
    Rdfs11,
    /// Container membership properties are subproperties of member.
    Rdfs12,
    /// Datatypes are subclasses of Literal.
    Rdfs13,
    /// Symmetric property assertions, OWL 2 RL rule prp-symp.
    PrpSymp,
    /// Transitive property assertions, OWL 2 RL rule prp-trp.
    PrpTrp,
    /// Forward inverse property assertions, OWL 2 RL rule prp-inv1.
    PrpInv1,
    /// Reverse inverse property assertions, OWL 2 RL rule prp-inv2.
    PrpInv2,
    /// Equality reflexivity for an assertion subject.
    EqRefSubject,
    /// Equality reflexivity for an assertion predicate.
    EqRefPredicate,
    /// Equality reflexivity for an assertion object, including a literal.
    EqRefObject,
    /// Equality symmetry.
    EqSym,
    /// Equality transitivity.
    EqTrans,
    /// Replace an assertion subject using exact equality.
    EqRepS,
    /// Replace an assertion predicate using exact equality.
    EqRepP,
    /// Replace an assertion object using exact equality.
    EqRepO,
    /// Functional properties imply equal objects.
    PrpFp,
    /// Inverse-functional properties imply equal subjects.
    PrpIfp,
    /// Equivalent properties transfer assertions forward.
    PrpEqp1,
    /// Equivalent properties transfer assertions in reverse.
    PrpEqp2,
    /// A typed property filler implies someValuesFrom membership.
    ClsSvf1,
    /// Any property filler satisfies someValuesFrom Thing.
    ClsSvf2,
    /// allValuesFrom membership types a property filler.
    ClsAvf,
    /// hasValue membership implies an assertion.
    ClsHv1,
    /// An assertion implies hasValue membership.
    ClsHv2,
    /// Equivalent classes transfer membership forward.
    CaxEqc1,
    /// Equivalent classes transfer membership in reverse.
    CaxEqc2,
    /// A declared OWL class is its own subclass.
    ScmClsSelfSubclass,
    /// A declared OWL class is equivalent to itself.
    ScmClsSelfEquivalent,
    /// A declared OWL class is a subclass of Thing.
    ScmClsThing,
    /// Nothing is a subclass of every declared OWL class.
    ScmClsNothing,
    /// Class equivalence implies forward subclass inclusion.
    ScmEqc1Forward,
    /// Class equivalence implies reverse subclass inclusion.
    ScmEqc1Reverse,
    /// Mutual subclass inclusion implies class equivalence.
    ScmEqc2,
    /// Property equivalence implies forward subproperty inclusion.
    ScmEqp1Forward,
    /// Property equivalence implies reverse subproperty inclusion.
    ScmEqp1Reverse,
    /// Mutual subproperty inclusion implies property equivalence.
    ScmEqp2,
    /// A property domain propagates along subclass inclusion.
    ScmDom1,
    /// A property domain propagates to a subproperty.
    ScmDom2,
    /// A property range propagates along subclass inclusion.
    ScmRng1,
    /// A property range propagates to a subproperty.
    ScmRng2,
    /// hasValue restrictions propagate along subproperty inclusion.
    ScmHv,
    /// someValuesFrom restrictions are covariant in their filler.
    ScmSvf1,
    /// someValuesFrom restrictions are covariant in their property.
    ScmSvf2,
    /// allValuesFrom restrictions are covariant in their filler.
    ScmAvf1,
    /// allValuesFrom restrictions are contravariant in their property.
    ScmAvf2,
    /// A declared object property is its own subproperty.
    ScmOpSelfSubproperty,
    /// A declared object property is equivalent to itself.
    ScmOpSelfEquivalent,
    /// A declared datatype property is its own subproperty.
    ScmDpSelfSubproperty,
    /// A declared datatype property is equivalent to itself.
    ScmDpSelfEquivalent,
}

impl RuleId {
    /// Whether this clause generates or applies OWL identity equality.
    ///
    /// This includes functional/inverse-functional property consequences.
    /// It does not classify arbitrary assertions whose predicate happens to
    /// be owl:sameAs, or equivalent-class/property schema relationships.
    #[must_use]
    pub const fn has_owl_equality_semantics(self) -> bool {
        matches!(
            self,
            Self::EqRefSubject
                | Self::EqRefPredicate
                | Self::EqRefObject
                | Self::EqSym
                | Self::EqTrans
                | Self::EqRepS
                | Self::EqRepP
                | Self::EqRepO
                | Self::PrpFp
                | Self::PrpIfp
        )
    }

    /// Every implemented clause, including clauses outside the initial presets.
    pub const ALL: &'static [Self] = &[
        Self::Rdf1,
        Self::Rdfs2,
        Self::Rdfs3,
        Self::Rdfs4a,
        Self::Rdfs4b,
        Self::Rdfs5,
        Self::Rdfs6,
        Self::Rdfs7,
        Self::Rdfs8,
        Self::Rdfs9,
        Self::Rdfs10,
        Self::Rdfs11,
        Self::Rdfs12,
        Self::Rdfs13,
        Self::PrpSymp,
        Self::PrpTrp,
        Self::PrpInv1,
        Self::PrpInv2,
        Self::EqRefSubject,
        Self::EqRefPredicate,
        Self::EqRefObject,
        Self::EqSym,
        Self::EqTrans,
        Self::EqRepS,
        Self::EqRepP,
        Self::EqRepO,
        Self::PrpFp,
        Self::PrpIfp,
        Self::PrpEqp1,
        Self::PrpEqp2,
        Self::ClsSvf1,
        Self::ClsSvf2,
        Self::ClsAvf,
        Self::ClsHv1,
        Self::ClsHv2,
        Self::CaxEqc1,
        Self::CaxEqc2,
        Self::ScmClsSelfSubclass,
        Self::ScmClsSelfEquivalent,
        Self::ScmClsThing,
        Self::ScmClsNothing,
        Self::ScmEqc1Forward,
        Self::ScmEqc1Reverse,
        Self::ScmEqc2,
        Self::ScmEqp1Forward,
        Self::ScmEqp1Reverse,
        Self::ScmEqp2,
        Self::ScmDom1,
        Self::ScmDom2,
        Self::ScmRng1,
        Self::ScmRng2,
        Self::ScmHv,
        Self::ScmSvf1,
        Self::ScmSvf2,
        Self::ScmAvf1,
        Self::ScmAvf2,
        Self::ScmOpSelfSubproperty,
        Self::ScmOpSelfEquivalent,
        Self::ScmDpSelfSubproperty,
        Self::ScmDpSelfEquivalent,
    ];

    /// Construct the actual checked Datalog clause for this rule.
    ///
    /// Rule constants are full vocabulary IRIs. Runtime adapters must bind these
    /// as IRIs, preserving the identity of all other caller-owned RDF terms.
    #[must_use]
    pub fn definition(self) -> Rule {
        let (head, body): ([&str; 3], &[[&str; 3]]) = match self {
            Self::Rdf1 => (["?p", "rdf:type", "rdf:Property"], &[["?s", "?p", "?o"]]),
            Self::Rdfs2 => (
                ["?s", "rdf:type", "?c"],
                &[["?p", "rdfs:domain", "?c"], ["?s", "?p", "?o"]],
            ),
            Self::Rdfs3 => (
                ["?o", "rdf:type", "?c"],
                &[["?p", "rdfs:range", "?c"], ["?s", "?p", "?o"]],
            ),
            Self::Rdfs4a => (["?s", "rdf:type", "rdfs:Resource"], &[["?s", "?p", "?o"]]),
            Self::Rdfs4b => (["?o", "rdf:type", "rdfs:Resource"], &[["?s", "?p", "?o"]]),
            Self::Rdfs5 => (
                ["?p", "rdfs:subPropertyOf", "?r"],
                &[
                    ["?p", "rdfs:subPropertyOf", "?q"],
                    ["?q", "rdfs:subPropertyOf", "?r"],
                ],
            ),
            Self::Rdfs6 => (
                ["?p", "rdfs:subPropertyOf", "?p"],
                &[["?p", "rdf:type", "rdf:Property"]],
            ),
            Self::Rdfs7 => (
                ["?s", "?q", "?o"],
                &[["?p", "rdfs:subPropertyOf", "?q"], ["?s", "?p", "?o"]],
            ),
            Self::Rdfs8 => (
                ["?c", "rdfs:subClassOf", "rdfs:Resource"],
                &[["?c", "rdf:type", "rdfs:Class"]],
            ),
            Self::Rdfs9 => (
                ["?s", "rdf:type", "?d"],
                &[["?s", "rdf:type", "?c"], ["?c", "rdfs:subClassOf", "?d"]],
            ),
            Self::Rdfs10 => (
                ["?c", "rdfs:subClassOf", "?c"],
                &[["?c", "rdf:type", "rdfs:Class"]],
            ),
            Self::Rdfs11 => (
                ["?c", "rdfs:subClassOf", "?e"],
                &[
                    ["?c", "rdfs:subClassOf", "?d"],
                    ["?d", "rdfs:subClassOf", "?e"],
                ],
            ),
            Self::Rdfs12 => (
                ["?p", "rdfs:subPropertyOf", "rdfs:member"],
                &[["?p", "rdf:type", "rdfs:ContainerMembershipProperty"]],
            ),
            Self::Rdfs13 => (
                ["?d", "rdfs:subClassOf", "rdfs:Literal"],
                &[["?d", "rdf:type", "rdfs:Datatype"]],
            ),
            Self::PrpSymp => (
                ["?o", "?p", "?s"],
                &[
                    ["?p", "rdf:type", "owl:SymmetricProperty"],
                    ["?s", "?p", "?o"],
                ],
            ),
            Self::PrpTrp => (
                ["?s", "?p", "?o"],
                &[
                    ["?p", "rdf:type", "owl:TransitiveProperty"],
                    ["?s", "?p", "?m"],
                    ["?m", "?p", "?o"],
                ],
            ),
            Self::PrpInv1 => (
                ["?o", "?q", "?s"],
                &[["?p", "owl:inverseOf", "?q"], ["?s", "?p", "?o"]],
            ),
            Self::PrpInv2 => (
                ["?o", "?p", "?s"],
                &[["?p", "owl:inverseOf", "?q"], ["?s", "?q", "?o"]],
            ),
            _ => return self.owl_definition(),
        };
        Pattern(head).rule(body)
    }
}

pub(super) struct Pattern<'a>(pub(super) [&'a str; 3]);

impl Pattern<'_> {
    fn rule(self, body: &[[&str; 3]]) -> Rule {
        // bounded: built-in clauses have at most five three-place body atoms.
        Rule::try_new(
            self.atom(),
            body.iter()
                .map(|terms| Literal::Positive(Pattern(*terms).atom()))
                .collect(),
        )
        .expect("the built-in RDF rule catalogue must be safe Datalog")
    }

    pub(super) fn atom(self) -> Atom {
        // bounded: exactly three terms per RDF assertion.
        Atom {
            predicate: PredicateSymbol::new(TRIPLE_RELATION),
            terms: self.0.into_iter().map(Self::term).collect(),
        }
    }

    pub(super) fn term(text: &str) -> Term {
        if let Some(variable) = text.strip_prefix('?') {
            return Term::Variable(VariableSymbol::new(variable));
        }
        let (prefix, local) = text
            .split_once(':')
            .expect("catalogue constants are namespaced IRIs");
        let namespace = match prefix {
            "rdf" => "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
            "rdfs" => "http://www.w3.org/2000/01/rdf-schema#",
            "owl" => "http://www.w3.org/2002/07/owl#",
            _ => panic!("unknown catalogue namespace {prefix}"),
        };
        Term::Constant(ConstantSymbol::new(format!("{namespace}{local}")))
    }
}
