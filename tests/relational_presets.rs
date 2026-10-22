//! Explicit restricted flavours must select concrete clauses and omissions.

mod support;

use rdf_reasoning::{Exclusion, Preset, RuleId};
use support::Fixture;

#[test]
fn relational_flavours_have_exact_rules_and_exclusions() {
    let with = Preset::RdfsOwl2RlRelationalCore;
    let without = Preset::RdfsOwl2RlRelationalCoreWithoutOwlEqualityRules;
    assert_eq!(with.rules().len(), 60);
    assert_eq!(without.rules().len(), 50);
    assert_eq!(with.rules(), RuleId::ALL);
    for id in with.rules() {
        assert_eq!(
            without.rules().contains(id),
            !id.has_owl_equality_semantics()
        );
    }
    for excluded in [
        Exclusion::ListRules,
        Exclusion::CardinalityRules,
        Exclusion::InconsistencyChecks,
    ] {
        assert!(with.exclusions().contains(&excluded));
        assert!(without.exclusions().contains(&excluded));
    }
    assert!(!with.exclusions().contains(&Exclusion::Equality));
    assert!(without.exclusions().contains(&Exclusion::Equality));
    assert!(without.rules().contains(&RuleId::PrpSymp));
    assert!(without.rules().contains(&RuleId::ClsHv1));
}

#[test]
fn disabling_owl_equality_rules_removes_replacement_and_functional_generation() {
    for (input, conclusion) in [
        (vec!["a owl:sameAs b", "a p c"], "b p c"),
        (
            vec!["p rdf:type owl:FunctionalProperty", "a p b", "a p c"],
            "b owl:sameAs c",
        ),
    ] {
        for (preset, expected) in [
            (Preset::RdfsOwl2RlRelationalCore, true),
            (
                Preset::RdfsOwl2RlRelationalCoreWithoutOwlEqualityRules,
                false,
            ),
        ] {
            assert_eq!(
                preset
                    .rules()
                    .iter()
                    .any(|id| Fixture::apply(&id.definition(), &input)
                        .contains(&Fixture::triple(conclusion))),
                expected
            );
        }
    }
}

#[test]
fn restricted_selectors_are_exact_and_do_not_alias_complete_profiles() {
    for (name, preset) in [
        (
            "rdfs-owl2-rl-relational-core",
            Preset::RdfsOwl2RlRelationalCore,
        ),
        (
            "rdfs-owl2-rl-relational-core-without-owl-equality-rules",
            Preset::RdfsOwl2RlRelationalCoreWithoutOwlEqualityRules,
        ),
    ] {
        assert_eq!(name.parse(), Ok(preset));
        assert_eq!(preset.name(), name);
        assert_eq!(
            serde_json::to_string(&preset).unwrap(),
            format!("\"{name}\"")
        );
    }
    for name in ["owl2-rl", "owl-horst", "owl2-rl-no-sameas", "owlrl-7.6.2"] {
        assert!(name.parse::<Preset>().is_err());
    }
}
