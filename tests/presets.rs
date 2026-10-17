//! Preset identity, exact coverage and serialization contracts.

use rdf_reasoning::{Preset, RuleId};

#[test]
fn exact_catalogue_and_exclusions() {
    assert!(Preset::None.rules().is_empty());
    assert_eq!(Preset::RdfsCore.rules().len(), 14);
    assert_eq!(Preset::RdfsPlusCore.rules().len(), 18);
    assert!(!Preset::RdfsCore.rules().contains(&RuleId::PrpSymp));
    assert!(Preset::RdfsPlusCore.rules().contains(&RuleId::PrpSymp));
    assert_eq!(Preset::RdfsCore.exclusions().len(), 5);
}

#[test]
fn reject_unimplemented_standard_and_vendor_names() {
    for name in [
        "rdfs",
        "owl2-rl",
        "owl-horst",
        "graphdb-11.3-owl-horst",
        "rdfs-core,off",
    ] {
        assert!(name.parse::<Preset>().is_err(), "{name}");
    }
}

#[test]
fn checked_positive_datalog_for_every_preset() {
    for preset in Preset::ALL {
        let program = preset.program();
        assert_eq!(program.rules.len(), preset.rules().len());
        program
            .validate_for(&datalog_structural_model::DatalogProfile::positive())
            .unwrap();
    }
}

#[test]
fn text_and_binary_round_trip() {
    for preset in Preset::ALL {
        assert_eq!(
            serde_json::from_str::<Preset>(&serde_json::to_string(&preset).unwrap()).unwrap(),
            *preset
        );
        assert_eq!(
            postcard::from_bytes::<Preset>(&postcard::to_allocvec(&preset).unwrap()).unwrap(),
            *preset
        );
    }
    for rule in Preset::RdfsPlusCore.rules() {
        assert_eq!(
            postcard::from_bytes::<RuleId>(&postcard::to_allocvec(rule).unwrap()).unwrap(),
            *rule
        );
    }
    assert!(serde_json::from_str::<Preset>(r#""Rdfs""#).is_err());
}

#[test]
fn compiled_program_round_trips_in_both_formats() {
    for preset in Preset::ALL {
        let program = preset.program();
        let json = serde_json::to_string(&program).unwrap();
        assert_eq!(
            serde_json::from_str::<datalog_structural_model::Program>(&json).unwrap(),
            program
        );
        let bytes = postcard::to_allocvec(&program).unwrap();
        assert_eq!(
            postcard::from_bytes::<datalog_structural_model::Program>(&bytes).unwrap(),
            program
        );
        for excluded in preset.exclusions() {
            let bytes = postcard::to_allocvec(excluded).unwrap();
            assert_eq!(
                postcard::from_bytes::<rdf_reasoning::Exclusion>(&bytes).unwrap(),
                *excluded
            );
        }
        assert_eq!(preset.name().parse(), Ok(*preset));
        assert_eq!(
            serde_json::to_string(preset).unwrap(),
            format!("\"{}\"", preset.name())
        );
    }
}

#[test]
fn rejected_selection_preserves_exact_diagnostics() {
    let error = "OWL2-RL".parse::<Preset>().unwrap_err();
    assert_eq!(error.input(), "OWL2-RL");
    let json = serde_json::to_string(&error).unwrap();
    assert_eq!(
        serde_json::from_str::<rdf_reasoning::PresetParseError>(&json).unwrap(),
        error
    );
    let bytes = postcard::to_allocvec(&error).unwrap();
    assert_eq!(
        postcard::from_bytes::<rdf_reasoning::PresetParseError>(&bytes).unwrap(),
        error
    );
    assert!(
        serde_json::from_str::<rdf_reasoning::PresetParseError>(
            r#"{"input":"rdfs", "ignored":true}"#
        )
        .is_err()
    );
}
