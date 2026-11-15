//! Finite empty-body clauses from OWL 2 RL/RDF, Second Edition (2012).
use crate::rule::Pattern;
use datalog_structural_model::Rule;

/// Exact identities of the four OWL 2 RL/RDF ground-axiom families.
///
/// These declarations do not implement datatype interpretation or alter existing
/// presets. They exclude the optional RDF/RDFS axiomatic triples.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum OwlRlAxiomId {
    /// Declare the nine built-in annotation properties (`prp-ap`).
    PrpAp,
    /// Declare `owl:Thing` as an OWL class (`cls-thing`).
    ClsThing,
    /// Declare `owl:Nothing` as an OWL class (`cls-nothing1`).
    ClsNothing1,
    /// Declare all 32 supported OWL-RL datatypes (`dt-type1`).
    DtType1,
}

impl OwlRlAxiomId {
    /// All four finite ground-axiom families, independent of restricted presets.
    pub const ALL: &'static [Self] = &[
        Self::PrpAp,
        Self::ClsThing,
        Self::ClsNothing1,
        Self::DtType1,
    ];

    /// Construct checked ground facts with empty bodies.
    ///
    /// The 2012 datatype inventory is a declaration contract, not evidence that
    /// an executor can interpret values in every declared datatype.
    #[must_use]
    pub fn definitions(self) -> Vec<Rule> {
        let (subjects, class): (&[&str], &str) = match self {
            Self::PrpAp => (
                &[
                    "rdfs:label",
                    "rdfs:comment",
                    "rdfs:seeAlso",
                    "rdfs:isDefinedBy",
                    "owl:deprecated",
                    "owl:versionInfo",
                    "owl:priorVersion",
                    "owl:backwardCompatibleWith",
                    "owl:incompatibleWith",
                ],
                "owl:AnnotationProperty",
            ),
            Self::ClsThing => (&["owl:Thing"], "owl:Class"),
            Self::ClsNothing1 => (&["owl:Nothing"], "owl:Class"),
            Self::DtType1 => (
                &[
                    "rdf:PlainLiteral",
                    "rdf:XMLLiteral",
                    "rdfs:Literal",
                    "xsd:decimal",
                    "xsd:integer",
                    "xsd:nonNegativeInteger",
                    "xsd:nonPositiveInteger",
                    "xsd:positiveInteger",
                    "xsd:negativeInteger",
                    "xsd:long",
                    "xsd:int",
                    "xsd:short",
                    "xsd:byte",
                    "xsd:unsignedLong",
                    "xsd:unsignedInt",
                    "xsd:unsignedShort",
                    "xsd:unsignedByte",
                    "xsd:float",
                    "xsd:double",
                    "xsd:string",
                    "xsd:normalizedString",
                    "xsd:token",
                    "xsd:language",
                    "xsd:Name",
                    "xsd:NCName",
                    "xsd:NMTOKEN",
                    "xsd:boolean",
                    "xsd:hexBinary",
                    "xsd:base64Binary",
                    "xsd:anyURI",
                    "xsd:dateTime",
                    "xsd:dateTimeStamp",
                ],
                "rdfs:Datatype",
            ),
        };
        // bounded: at most 32 ground facts from a fixed standard vocabulary.
        subjects
            .iter()
            .map(|subject| {
                Rule::try_new(Pattern([subject, "rdf:type", class]).atom(), Vec::new())
                    .expect("OWL-RL ground axioms must be checked safe Datalog facts")
            })
            .collect()
    }
}
