//! Exact shared subclass contract for the first inference-engine consumer.
#[path = "support/relations.rs"]
mod relations;

use rdf_reasoning::{Exclusion, Preset, RuleId, TRIPLE_RELATION};
use relations::{Fact, closure};

const TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const SUB: &str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";

#[test]
fn subclass_selector_has_exactly_two_shared_clauses_and_explicit_exclusions() {
    let preset = Preset::RdfsSubclass;
    assert_eq!("rdfs-subclass".parse(), Ok(preset));
    assert_eq!(preset.name(), "rdfs-subclass");
    assert_eq!(preset.rules(), [RuleId::Rdfs9, RuleId::Rdfs11]);
    assert_eq!(
        preset.program().rules,
        [RuleId::Rdfs9.definition(), RuleId::Rdfs11.definition()]
    );
    assert_eq!(
        preset.exclusions(),
        [
            Exclusion::OtherRdfsRules,
            Exclusion::OwlRules,
            Exclusion::AxiomaticTriples,
            Exclusion::DatatypeSemantics,
            Exclusion::ContainerVocabularySeeding,
            Exclusion::SimpleEntailmentChecking,
            Exclusion::Equality,
            Exclusion::ListRules,
            Exclusion::CardinalityRules,
            Exclusion::InconsistencyChecks,
        ]
    );
    assert!(Preset::ALL.contains(&preset));
    for old in [
        Preset::RdfsCore,
        Preset::RdfsPlusCore,
        Preset::RdfsOwl2RlRelationalCore,
    ] {
        assert!(!old.exclusions().contains(&Exclusion::OtherRdfsRules));
    }
}

#[test]
fn subclass_fixed_point_preserves_join_identity_and_does_not_run_other_rdfs_rules() {
    let data = [
        Fact::new(TRIPLE_RELATION, &["urn:x", TYPE, "urn:A"]),
        Fact::new(TRIPLE_RELATION, &["urn:A", SUB, "urn:B"]),
        Fact::new(TRIPLE_RELATION, &["urn:B", SUB, "urn:C"]),
        Fact::new(TRIPLE_RELATION, &["urn:C", SUB, "urn:A"]),
        Fact::new(TRIPLE_RELATION, &["urn:D", SUB, "urn:E"]),
        Fact::new(
            TRIPLE_RELATION,
            &[
                "urn:p",
                "http://www.w3.org/2000/01/rdf-schema#domain",
                "urn:D",
            ],
        ),
        Fact::new(TRIPLE_RELATION, &["urn:x", "urn:p", "urn:y"]),
    ];
    let program = Preset::RdfsSubclass.program();
    let result = closure(&program.rules, &data);
    for class in ["urn:A", "urn:B", "urn:C"] {
        assert!(result.contains(&Fact::new(TRIPLE_RELATION, &["urn:x", TYPE, class])));
        assert!(!result.contains(&Fact::new(TRIPLE_RELATION, &["urn:y", TYPE, class])));
    }
    assert!(result.contains(&Fact::new(TRIPLE_RELATION, &["urn:A", SUB, "urn:C"])));
    assert!(!result.contains(&Fact::new(TRIPLE_RELATION, &["urn:x", TYPE, "urn:D"])));
    assert!(!result.contains(&Fact::new(
        TRIPLE_RELATION,
        &[
            "urn:x",
            TYPE,
            "http://www.w3.org/2000/01/rdf-schema#Resource"
        ]
    )));
    assert!(!result.contains(&Fact::new(TRIPLE_RELATION, &["urn:A", SUB, "urn:E"])));
    assert!(closure(&program.rules, &[]).is_empty());
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Selection {
    preset: Preset,
    exclusion: Exclusion,
}

#[test]
fn nested_text_and_binary_selection_are_checked() {
    let selection = Selection {
        preset: Preset::RdfsSubclass,
        exclusion: Exclusion::OtherRdfsRules,
    };
    assert_eq!(
        serde_json::from_str::<Selection>(&serde_json::to_string(&selection).unwrap()).unwrap(),
        selection
    );
    assert_eq!(
        postcard::from_bytes::<Selection>(&postcard::to_allocvec(&selection).unwrap()).unwrap(),
        selection
    );
    for malformed in [
        r#"{"preset":"rdfs-subclasses","exclusion":"other-rdfs-rules"}"#,
        r#"{"preset":"rdfs-subclass","exclusion":"unknown"}"#,
        r#"{"preset":"rdfs-subclass","exclusion":"other-rdfs-rules","extra":true}"#,
    ] {
        assert!(serde_json::from_str::<Selection>(malformed).is_err());
    }
    assert!("rdfs".parse::<Preset>().is_err());
}
