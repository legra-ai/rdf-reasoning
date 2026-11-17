//! Checked authored fixture metadata, not a Turtle parser or entailment executor.
use std::collections::BTreeSet;

const DATA: &str = include_str!("fixtures/rdf12-entailment.json");
const SEMANTICS: &str = "https://www.w3.org/TR/2026/CRD-rdf12-semantics-20260924/";
const SYNTAX: &str = "https://www.w3.org/TR/2026/WD-rdf12-turtle-20261111/";

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "ManifestInput", into = "ManifestInput")]
struct Manifest(ManifestInput);

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestInput {
    format: u32,
    revision: u32,
    license: String,
    provenance: Provenance,
    semantics: String,
    syntax: String,
    entailment: Entailment,
    blank_node_scopes: BlankScopes,
    cases: Vec<Case>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Provenance {
    AuthoredRegression,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Entailment {
    Rdfs12Default,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum BlankScopes {
    IndependentSourceTarget,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    id: String,
    source_turtle: String,
    target_turtle: String,
    expected: bool,
    requires: Vec<Requirement>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Requirement {
    RecursiveOccurrences,
    PropositionConstruction,
    ConsistentSimpleMatching,
    ContainerSeeding,
    TargetContext,
    DefaultDatatypeWitnesses,
}

impl From<Manifest> for ManifestInput {
    fn from(value: Manifest) -> Self {
        value.0
    }
}

impl TryFrom<ManifestInput> for Manifest {
    type Error = &'static str;
    fn try_from(value: ManifestInput) -> Result<Self, Self::Error> {
        if value.format != 1
            || value.revision == 0
            || value.license != "MIT OR Apache-2.0"
            || value.semantics != SEMANTICS
            || value.syntax != SYNTAX
        {
            return Err("unsupported fixture identity, edition or license");
        }
        if value.cases.is_empty() || value.cases.len() > 32 {
            return Err("fixture case budget");
        }
        // bounded: at most 32 checked fixture cases, never caller graph data.
        let mut ids = BTreeSet::new();
        for case in &value.cases {
            if case.id.is_empty()
                || case.id.len() > 96
                || !case
                    .id
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
                || !ids.insert(&case.id)
            {
                return Err("invalid or duplicate fixture ID");
            }
            if case.source_turtle.len() > 8192
                || case.target_turtle.is_empty()
                || case.target_turtle.len() > 8192
                || case.requires.is_empty()
                || case.requires.len() > 6
            {
                return Err("fixture document or requirement budget");
            }
            // bounded: at most six explicitly authored requirements per case.
            let unique: BTreeSet<_> = case.requires.iter().collect();
            if unique.len() != case.requires.len() {
                return Err("duplicate fixture requirement");
            }
        }
        Ok(Self(value))
    }
}

#[test]
fn packaged_fixture_manifest_has_checked_identity_and_nested_round_trips() {
    assert!(DATA.len() <= 65536, "finite fixture bundle ceiling");
    let manifest: Manifest = serde_json::from_str(DATA).unwrap();
    assert!(manifest.0.cases.iter().any(|case| case.expected));
    assert!(manifest.0.cases.iter().any(|case| !case.expected));
    assert_eq!(
        serde_json::from_str::<Manifest>(&serde_json::to_string(&manifest).unwrap()).unwrap(),
        manifest
    );
    assert_eq!(
        postcard::from_bytes::<Manifest>(&postcard::to_allocvec(&manifest).unwrap()).unwrap(),
        manifest
    );
}

#[test]
fn unknown_fields_requirements_and_ambiguous_scopes_are_rejected() {
    for broken in [
        DATA.replacen("\"format\":", "\"extra\":true,\"format\":", 1),
        DATA.replace("recursive-occurrences", "unknown-requirement"),
        DATA.replace("independent-source-target", "shared-source-target"),
        DATA.replacen("\"source_turtle\":", "\"extra\":true,\"source_turtle\":", 1),
    ] {
        assert!(serde_json::from_str::<Manifest>(&broken).is_err());
    }
}

#[test]
fn duplicate_ids_requirements_and_excessive_documents_fail_checked_transport() {
    let manifest: Manifest = serde_json::from_str(DATA).unwrap();
    let mut duplicate_id = manifest.0.clone();
    duplicate_id.cases.push(duplicate_id.cases[0].clone());
    let mut duplicate_requirement = manifest.0.clone();
    let repeated = duplicate_requirement.cases[0].requires[0].clone();
    duplicate_requirement.cases[0].requires.push(repeated);
    let mut excessive_document = manifest.0;
    excessive_document.cases[0].source_turtle = "x".repeat(8193);
    for input in [duplicate_id, duplicate_requirement, excessive_document] {
        assert!(serde_json::from_str::<Manifest>(&serde_json::to_string(&input).unwrap()).is_err());
        assert!(postcard::from_bytes::<Manifest>(&postcard::to_allocvec(&input).unwrap()).is_err());
    }
}
