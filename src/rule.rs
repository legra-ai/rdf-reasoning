//! Finite, independently authored relational rule catalogue.

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
}

impl RuleId {
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
        };
        // bounded: at most three body atoms, each with exactly three terms.
        Rule::try_new(
            Pattern(head).atom(),
            body.iter()
                .map(|terms| Literal::Positive(Pattern(*terms).atom()))
                .collect(),
        )
        .expect("the built-in RDF rule catalogue must be safe Datalog")
    }
}

struct Pattern<'a>([&'a str; 3]);

impl Pattern<'_> {
    fn atom(self) -> Atom {
        // bounded: exactly three terms per RDF assertion.
        Atom {
            predicate: PredicateSymbol::new(TRIPLE_RELATION),
            terms: self.0.into_iter().map(Self::term).collect(),
        }
    }

    fn term(text: &str) -> Term {
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
