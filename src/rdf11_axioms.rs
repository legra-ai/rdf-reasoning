//! Finite ground axioms from RDF 1.1 Semantics (2014).
use crate::rule::Pattern;
use datalog_structural_model::Rule;

/// Exact finite seed families, separate from structural rule presets.
///
/// This does not enumerate the infinite `rdf:_n` vocabulary. Entailment checking
/// must seed target-relevant container IRIs through [`crate::Rdf11ContainerProperty`].
/// No selection here establishes complete RDFS execution or datatype semantics.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Rdf11AxiomId {
    /// Eight non-container RDF axioms from section 8.
    Rdf,
    /// Thirty-eight non-container RDFS axioms from section 9.
    Rdfs,
    /// The two recognized default datatype declarations (`rdfs1`).
    DefaultDatatypes,
}

impl Rdf11AxiomId {
    /// The three explicit finite seed families.
    pub const ALL: &'static [Self] = &[Self::Rdf, Self::Rdfs, Self::DefaultDatatypes];

    /// Checked ground facts for one selected finite family.
    ///
    /// Default declarations recognize only `xsd:string` and `rdf:langString`.
    /// Encountering another datatype in input does not recognize it, and these
    /// declarations do not validate literal lexical forms or certify consistency.
    #[must_use]
    pub fn definitions(self) -> Vec<Rule> {
        let triples: &[[&str; 3]] = match self {
            Self::Rdf => &[
                ["rdf:type", "rdf:type", "rdf:Property"],
                ["rdf:subject", "rdf:type", "rdf:Property"],
                ["rdf:predicate", "rdf:type", "rdf:Property"],
                ["rdf:object", "rdf:type", "rdf:Property"],
                ["rdf:first", "rdf:type", "rdf:Property"],
                ["rdf:rest", "rdf:type", "rdf:Property"],
                ["rdf:value", "rdf:type", "rdf:Property"],
                ["rdf:nil", "rdf:type", "rdf:List"],
            ],
            Self::Rdfs => &[
                ["rdf:type", "rdfs:domain", "rdfs:Resource"],
                ["rdfs:domain", "rdfs:domain", "rdf:Property"],
                ["rdfs:range", "rdfs:domain", "rdf:Property"],
                ["rdfs:subPropertyOf", "rdfs:domain", "rdf:Property"],
                ["rdfs:subClassOf", "rdfs:domain", "rdfs:Class"],
                ["rdf:subject", "rdfs:domain", "rdf:Statement"],
                ["rdf:predicate", "rdfs:domain", "rdf:Statement"],
                ["rdf:object", "rdfs:domain", "rdf:Statement"],
                ["rdfs:member", "rdfs:domain", "rdfs:Resource"],
                ["rdf:first", "rdfs:domain", "rdf:List"],
                ["rdf:rest", "rdfs:domain", "rdf:List"],
                ["rdfs:seeAlso", "rdfs:domain", "rdfs:Resource"],
                ["rdfs:isDefinedBy", "rdfs:domain", "rdfs:Resource"],
                ["rdfs:comment", "rdfs:domain", "rdfs:Resource"],
                ["rdfs:label", "rdfs:domain", "rdfs:Resource"],
                ["rdf:value", "rdfs:domain", "rdfs:Resource"],
                ["rdf:type", "rdfs:range", "rdfs:Class"],
                ["rdfs:domain", "rdfs:range", "rdfs:Class"],
                ["rdfs:range", "rdfs:range", "rdfs:Class"],
                ["rdfs:subPropertyOf", "rdfs:range", "rdf:Property"],
                ["rdfs:subClassOf", "rdfs:range", "rdfs:Class"],
                ["rdf:subject", "rdfs:range", "rdfs:Resource"],
                ["rdf:predicate", "rdfs:range", "rdfs:Resource"],
                ["rdf:object", "rdfs:range", "rdfs:Resource"],
                ["rdfs:member", "rdfs:range", "rdfs:Resource"],
                ["rdf:first", "rdfs:range", "rdfs:Resource"],
                ["rdf:rest", "rdfs:range", "rdf:List"],
                ["rdfs:seeAlso", "rdfs:range", "rdfs:Resource"],
                ["rdfs:isDefinedBy", "rdfs:range", "rdfs:Resource"],
                ["rdfs:comment", "rdfs:range", "rdfs:Literal"],
                ["rdfs:label", "rdfs:range", "rdfs:Literal"],
                ["rdf:value", "rdfs:range", "rdfs:Resource"],
                ["rdf:Alt", "rdfs:subClassOf", "rdfs:Container"],
                ["rdf:Bag", "rdfs:subClassOf", "rdfs:Container"],
                ["rdf:Seq", "rdfs:subClassOf", "rdfs:Container"],
                [
                    "rdfs:ContainerMembershipProperty",
                    "rdfs:subClassOf",
                    "rdf:Property",
                ],
                ["rdfs:isDefinedBy", "rdfs:subPropertyOf", "rdfs:seeAlso"],
                ["rdfs:Datatype", "rdfs:subClassOf", "rdfs:Class"],
            ],
            Self::DefaultDatatypes => &[
                ["xsd:string", "rdf:type", "rdfs:Datatype"],
                ["rdf:langString", "rdf:type", "rdfs:Datatype"],
            ],
        };
        // bounded: at most 38 fixed standard ground facts, independent of input.
        triples
            .iter()
            .map(|row| {
                Rule::try_new(Pattern(*row).atom(), Vec::new())
                    .expect("RDF 1.1 ground axioms must be checked safe Datalog facts")
            })
            .collect()
    }
}
