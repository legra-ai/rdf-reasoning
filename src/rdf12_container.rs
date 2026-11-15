//! Checked one-IRI seeds for the infinite RDF container vocabulary.
use crate::{TRIPLE_RELATION, rule::Pattern};
use datalog_structural_model::{Atom, ConstantSymbol, PredicateSymbol, Rule, Term};
use std::fmt;

const PREFIX: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#_";

/// A canonical positive `rdf:_n` IRI, preserving caller-owned or borrowed text.
///
/// The suffix has no leading zeros and is not parsed into a machine integer.
/// Selecting relevant IRIs is a consumer responsibility: Appendix A entailment
/// closure uses target vocabulary and `rdf:_1` when no target container is used.
/// Observed-input-only seeding does not certify complete entailment checking.
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[serde(
    try_from = "ContainerInput<S>",
    bound(deserialize = "S: serde::Deserialize<'de>")
)]
pub struct Rdf12ContainerProperty<S: AsRef<str> = String> {
    iri: S,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ContainerInput<S> {
    iri: S,
}

/// The supplied IRI is not a canonical positive RDF container property.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvalidRdf12ContainerProperty;

impl fmt::Display for InvalidRdf12ContainerProperty {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            "expected an absolute rdf:_n IRI with positive decimal n and no leading zeros",
        )
    }
}

impl std::error::Error for InvalidRdf12ContainerProperty {}

impl<S: AsRef<str>> Rdf12ContainerProperty<S> {
    /// Check the exact namespace and canonical unbounded decimal suffix.
    /// # Errors
    /// Rejects every non-container IRI and invalid suffix; never substitutes `_1`.
    pub fn try_new(iri: S) -> Result<Self, InvalidRdf12ContainerProperty> {
        let suffix = iri
            .as_ref()
            .strip_prefix(PREFIX)
            .ok_or(InvalidRdf12ContainerProperty)?;
        if !matches!(suffix.as_bytes().first(), Some(b'1'..=b'9'))
            || !suffix.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(InvalidRdf12ContainerProperty);
        }
        Ok(Self { iri })
    }

    /// The exact validated IRI, without copying its backing representation.
    #[must_use]
    pub fn iri(&self) -> &str {
        self.iri.as_ref()
    }

    /// One RDF axiom declaring this IRI as a property.
    #[must_use]
    pub fn rdf_definition(&self) -> Rule {
        self.fact("rdf:type", "rdf:Property")
    }

    /// Three RDFS axioms: membership-property typing, domain and range.
    ///
    /// Include the separate RDF axiom when computing RDF/RDFS closure.
    #[must_use]
    pub fn rdfs_definitions(&self) -> [Rule; 3] {
        [
            self.fact("rdf:type", "rdfs:ContainerMembershipProperty"),
            self.fact("rdfs:domain", "rdfs:Resource"),
            self.fact("rdfs:range", "rdfs:Resource"),
        ]
    }

    fn fact(&self, predicate: &str, object: &str) -> Rule {
        // bounded: three terms for one checked vocabulary IRI, never a graph.
        let head = Atom {
            predicate: PredicateSymbol::new(TRIPLE_RELATION),
            terms: vec![
                Term::Constant(ConstantSymbol::new(self.iri())),
                Pattern::term(predicate),
                Pattern::term(object),
            ],
        };
        Rule::try_new(head, Vec::new())
            .expect("checked container vocabulary axioms must be safe Datalog facts")
    }
}

impl<S: AsRef<str>> TryFrom<ContainerInput<S>> for Rdf12ContainerProperty<S> {
    type Error = InvalidRdf12ContainerProperty;

    fn try_from(input: ContainerInput<S>) -> Result<Self, Self::Error> {
        Self::try_new(input.iri)
    }
}
