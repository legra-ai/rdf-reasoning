//! Exact, restricted presets with no implicit standard-conformance claim.

use crate::RuleId;
use crate::RuleId::*;
use datalog_structural_model::Program;
use std::{fmt, str::FromStr};

/// Deliberate omissions from the core rule presets.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Exclusion {
    /// RDF/RDFS axiomatic triples are not injected.
    AxiomaticTriples,
    /// Datatype recognition, value equivalence and ill-typed literal checks.
    DatatypeSemantics,
    /// Automatic classification of rdf:_n vocabulary IRIs.
    ContainerVocabularySeeding,
    /// Existential simple-entailment witnesses and query entailment checking.
    SimpleEntailmentChecking,
    /// Dedicated OWL equality derivation, reflexivity and replacement rules.
    Equality,
    /// RDF list-dependent intersections, unions, enumerations, chains and keys.
    ListRules,
    /// Cardinality and qualified-cardinality restriction rules.
    CardinalityRules,
    /// Rules detecting inconsistent assertions or datatype assignments.
    InconsistencyChecks,
}

/// An exact, finite ruleset, separate from forward/backward execution strategy.
///
/// These deliberately restricted names do not denote full standard RDFS,
/// OWL 2 RL, Horst or any vendor-compatible reasoning implementation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Preset {
    /// No inferred assertions or axioms.
    None,
    /// Fourteen positive RDF/RDFS structural rules, with explicit exclusions.
    RdfsCore,
    /// RDFS core plus symmetric, transitive and inverse property assertions.
    RdfsPlusCore,
    /// All sixty implemented RDFS/OWL positive flat relational clauses.
    RdfsOwl2RlRelationalCore,
    /// The relational core excluding ten dedicated OWL identity-equality clauses.
    RdfsOwl2RlRelationalCoreWithoutOwlEqualityRules,
}

impl Preset {
    /// Every implemented preset.
    pub const ALL: &'static [Self] = &[
        Self::None,
        Self::RdfsCore,
        Self::RdfsPlusCore,
        Self::RdfsOwl2RlRelationalCore,
        Self::RdfsOwl2RlRelationalCoreWithoutOwlEqualityRules,
    ];

    const WITHOUT_OWL_EQUALITY: [RuleId; 50] = Self::without_owl_equality();

    const fn without_owl_equality() -> [RuleId; 50] {
        // bounded: filtering sixty built-in rule identifiers at compile time.
        let mut selected = [Rdf1; 50];
        let mut input = 0;
        let mut output = 0;
        while input < RuleId::ALL.len() {
            let rule = RuleId::ALL[input];
            if !rule.has_owl_equality_semantics() {
                selected[output] = rule;
                output += 1;
            }
            input += 1;
        }
        assert!(output == selected.len());
        selected
    }

    /// Stable, exact selector used by text APIs.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RdfsCore => "rdfs-core",
            Self::RdfsPlusCore => "rdfs-plus-core",
            Self::RdfsOwl2RlRelationalCore => "rdfs-owl2-rl-relational-core",
            Self::RdfsOwl2RlRelationalCoreWithoutOwlEqualityRules => {
                "rdfs-owl2-rl-relational-core-without-owl-equality-rules"
            }
        }
    }

    /// Exact selected rule identifiers, in deterministic compilation order.
    #[must_use]
    pub const fn rules(self) -> &'static [RuleId] {
        match self {
            Self::None => &[],
            Self::RdfsCore => &[
                Rdf1, Rdfs2, Rdfs3, Rdfs4a, Rdfs4b, Rdfs5, Rdfs6, Rdfs7, Rdfs8, Rdfs9, Rdfs10,
                Rdfs11, Rdfs12, Rdfs13,
            ],
            Self::RdfsPlusCore => &[
                Rdf1, Rdfs2, Rdfs3, Rdfs4a, Rdfs4b, Rdfs5, Rdfs6, Rdfs7, Rdfs8, Rdfs9, Rdfs10,
                Rdfs11, Rdfs12, Rdfs13, PrpSymp, PrpTrp, PrpInv1, PrpInv2,
            ],
            Self::RdfsOwl2RlRelationalCore => RuleId::ALL,
            Self::RdfsOwl2RlRelationalCoreWithoutOwlEqualityRules => &Self::WITHOUT_OWL_EQUALITY,
        }
    }

    /// Semantics intentionally absent from the core presets.
    ///
    /// `None` excludes all inference by definition and has no rule contract.
    #[must_use]
    pub const fn exclusions(self) -> &'static [Exclusion] {
        match self {
            Self::None => &[],
            Self::RdfsCore | Self::RdfsPlusCore => &[
                Exclusion::AxiomaticTriples,
                Exclusion::DatatypeSemantics,
                Exclusion::ContainerVocabularySeeding,
                Exclusion::SimpleEntailmentChecking,
                Exclusion::Equality,
            ],
            Self::RdfsOwl2RlRelationalCore => &[
                Exclusion::AxiomaticTriples,
                Exclusion::DatatypeSemantics,
                Exclusion::ContainerVocabularySeeding,
                Exclusion::SimpleEntailmentChecking,
                Exclusion::ListRules,
                Exclusion::CardinalityRules,
                Exclusion::InconsistencyChecks,
            ],
            Self::RdfsOwl2RlRelationalCoreWithoutOwlEqualityRules => &[
                Exclusion::AxiomaticTriples,
                Exclusion::DatatypeSemantics,
                Exclusion::ContainerVocabularySeeding,
                Exclusion::SimpleEntailmentChecking,
                Exclusion::Equality,
                Exclusion::ListRules,
                Exclusion::CardinalityRules,
                Exclusion::InconsistencyChecks,
            ],
        }
    }

    /// Compile the finite catalogue into a validated positive Datalog program.
    ///
    /// No graph is read or materialized. Callers supply execution and graph scope.
    #[must_use]
    pub fn program(self) -> Program {
        // bounded: the largest preset has sixty rules, independent of graph size.
        Program::try_new(self.rules().iter().map(|rule| rule.definition()).collect())
            .expect("the built-in RDF preset catalogue must be safe Datalog")
    }
}

impl fmt::Display for Preset {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// An unknown or unimplemented preset; no weaker preset is substituted.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresetParseError {
    input: String,
}

impl PresetParseError {
    /// The exact rejected selection.
    #[must_use]
    pub fn input(&self) -> &str {
        &self.input
    }
}

impl FromStr for Preset {
    type Err = PresetParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "none" => Ok(Self::None),
            "rdfs-core" => Ok(Self::RdfsCore),
            "rdfs-plus-core" => Ok(Self::RdfsPlusCore),
            "rdfs-owl2-rl-relational-core" => Ok(Self::RdfsOwl2RlRelationalCore),
            "rdfs-owl2-rl-relational-core-without-owl-equality-rules" => {
                Ok(Self::RdfsOwl2RlRelationalCoreWithoutOwlEqualityRules)
            }
            _ => Err(PresetParseError {
                input: input.to_owned(),
            }),
        }
    }
}

impl fmt::Display for PresetParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown or unimplemented RDF reasoning preset {:?}",
            self.input
        )
    }
}

impl std::error::Error for PresetParseError {}
