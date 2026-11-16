//! Nonempty-class requirements from RDF 1.2 Semantics CRD 2026-09-24.
use crate::Rdf12Datatype;

/// A finite existential requirement, separate from ordinary safe Datalog rules.
///
/// The executor supplies a fresh scoped blank witness with this class membership,
/// once per descriptor and immutable entailment attempt/snapshot. Persist that
/// binding in caller-owned storage and reuse it within the scope. Different
/// descriptor keys do not assert different denotations or a unique-name policy.
/// Never substitute a fixed user IRI, Skolem IRI or chosen lexical representative.
///
/// Preserve descriptor, ground witness membership, scope and input origin as
/// semantic evidence. No asserted RDF premise is required, even for empty input.
/// RDF uses the default-map `rdfD1a` requirements; RDFS also uses `rdfs14a`.
/// This descriptor does not allocate identities, introduce existential Datalog
/// syntax, execute a witness strategy or certify complete entailment checking.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Rdf12ExistentialAxiomId {
    /// Nonempty value space of one mandatory default recognized datatype.
    #[serde(rename = "rdfD1a")]
    RdfD1a(Rdf12Datatype),
    /// Nonempty proposition class, including when the source graph is empty.
    #[serde(rename = "rdfs14a")]
    Rdfs14a,
}

impl Rdf12ExistentialAxiomId {
    /// Exactly four requirements: three default datatype classes and propositions.
    pub const ALL: &'static [Self] = &[
        Self::RdfD1a(Rdf12Datatype::String),
        Self::RdfD1a(Rdf12Datatype::LangString),
        Self::RdfD1a(Rdf12Datatype::DirLangString),
        Self::Rdfs14a,
    ];

    /// Standard clause name; `rdfD1a` identity also includes its datatype parameter.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::RdfD1a(_) => "rdfD1a",
            Self::Rdfs14a => "rdfs14a",
        }
    }

    /// Class in the existential `(blank witness, rdf:type, class)` membership.
    #[must_use]
    pub const fn class_iri(self) -> &'static str {
        match self {
            Self::RdfD1a(datatype) => datatype.iri(),
            Self::Rdfs14a => "http://www.w3.org/2000/01/rdf-schema#Proposition",
        }
    }
}
