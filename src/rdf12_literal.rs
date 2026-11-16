//! Default-map GrdfD1 over explicit, validated literal-metadata premises.
use crate::{Rdf12Datatype, TRIPLE_RELATION, rule::Pattern};
use datalog_structural_model::{Atom, ConstantSymbol, Literal, PredicateSymbol, Rule, Term};

/// Auxiliary metadata relation `(exact_literal_key, declared_datatype_iri)`.
///
/// The adapter selects source, target-context or recursively occurring literals
/// in its immutable semantic scope. It validates standard metadata and recognized
/// lexical forms before exposing positive premises. Unknown encountered datatypes
/// remain opaque and do not match any default-map GrdfD1 definition.
///
/// These premises are not asserted RDF triples. Preserve occurrence/origin and
/// lexical-validation evidence separately; never fabricate graph premises or
/// project this auxiliary relation into RDF output. Unsupported relation binding
/// must fail explicitly before execution. Positive Datalog syntax admission alone
/// does not certify literal inspection, UTF-8 validation or semantic execution.
pub const RDF12_LITERAL_DATATYPE_RELATION: &str = "urn:rdf-reasoning:rdf12-literal-datatype";

/// Parameterized literal rules for the mandatory default RDF 1.2 datatype map.
///
/// This is separate from existing restricted presets. Every GrdfD1 instance fixes
/// its recognized datatype as a constant; input `rdf:type rdfs:Datatype` assertions
/// cannot extend recognition. Metadata inspection and execution remain caller-owned.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Rdf12LiteralRuleId {
    /// Infer generalized literal membership for one default recognized datatype.
    #[serde(rename = "grdf-d1")]
    GrdfD1(Rdf12Datatype),
}

impl Rdf12LiteralRuleId {
    /// Exactly three instances, one for each mandatory default datatype.
    pub const ALL: &'static [Self] = &[
        Self::GrdfD1(Rdf12Datatype::String),
        Self::GrdfD1(Rdf12Datatype::LangString),
        Self::GrdfD1(Rdf12Datatype::DirLangString),
    ];

    /// Standard clause name; semantic identity also includes [`Self::datatype`].
    #[must_use]
    pub const fn name(self) -> &'static str {
        "grdf-d1"
    }

    /// The exact recognized datatype parameter of this clause instance.
    #[must_use]
    pub const fn datatype(self) -> Rdf12Datatype {
        match self {
            Self::GrdfD1(datatype) => datatype,
        }
    }

    /// Construct the checked one-premise generalized literal-typing clause.
    ///
    /// The same rule can consume validated source or target-context metadata,
    /// according to the caller's explicit entailment phase. It does not choose
    /// that phase, certify consistency or introduce an existential witness.
    #[must_use]
    pub fn definition(self) -> Rule {
        let literal = Pattern::term("?literal");
        let datatype = Term::Constant(ConstantSymbol::new(self.datatype().iri()));
        // bounded: three head terms and two terms in one fixed metadata premise.
        let head = Atom {
            predicate: PredicateSymbol::new(TRIPLE_RELATION),
            terms: vec![literal.clone(), Pattern::term("rdf:type"), datatype.clone()],
        };
        let metadata = Atom {
            predicate: PredicateSymbol::new(RDF12_LITERAL_DATATYPE_RELATION),
            terms: vec![literal, datatype],
        };
        Rule::try_new(head, vec![Literal::Positive(metadata)])
            .expect("default-map GrdfD1 definitions must be checked safe Datalog")
    }
}
