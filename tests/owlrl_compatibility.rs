//! Versioned Python quirks remain separate from standard clauses and presets.

mod support;

use datalog_structural_model::Rule;
use rdf_reasoning::{Owlrl762RuleId, RuleId};
use support::Fixture;

#[test]
fn python_schema_guards_do_not_redefine_standard_reflexivity() {
    for (reference, standard, input, output) in [
        (
            Owlrl762RuleId::ScmEqc1Forward,
            RuleId::ScmEqc1Forward,
            vec!["C owl:equivalentClass C"],
            "C rdfs:subClassOf C",
        ),
        (
            Owlrl762RuleId::ScmEqc1Reverse,
            RuleId::ScmEqc1Reverse,
            vec!["C owl:equivalentClass C"],
            "C rdfs:subClassOf C",
        ),
        (
            Owlrl762RuleId::ScmEqp1Forward,
            RuleId::ScmEqp1Forward,
            vec!["p owl:equivalentProperty p"],
            "p rdfs:subPropertyOf p",
        ),
        (
            Owlrl762RuleId::ScmEqp1Reverse,
            RuleId::ScmEqp1Reverse,
            vec!["p owl:equivalentProperty p"],
            "p rdfs:subPropertyOf p",
        ),
        (
            Owlrl762RuleId::ScmSco,
            RuleId::Rdfs11,
            vec!["C rdfs:subClassOf D", "D rdfs:subClassOf C"],
            "C rdfs:subClassOf C",
        ),
        (
            Owlrl762RuleId::ScmSpo,
            RuleId::Rdfs5,
            vec!["p rdfs:subPropertyOf q", "q rdfs:subPropertyOf p"],
            "p rdfs:subPropertyOf p",
        ),
        (
            Owlrl762RuleId::ScmEqp2,
            RuleId::ScmEqp2,
            vec!["p rdfs:subPropertyOf p"],
            "p owl:equivalentProperty p",
        ),
    ] {
        let expected = Fixture::triple(output);
        assert!(Fixture::apply(&standard.definition(), &input).contains(&expected));
        assert!(!Fixture::apply(&reference.definition(), &input).contains(&expected));
    }
}

#[test]
fn reference_guards_preserve_nonreflexive_consequences() {
    for (reference, input, output) in [
        (
            Owlrl762RuleId::ScmEqc1Forward,
            vec!["C owl:equivalentClass D"],
            "C rdfs:subClassOf D",
        ),
        (
            Owlrl762RuleId::ScmEqc1Reverse,
            vec!["C owl:equivalentClass D"],
            "D rdfs:subClassOf C",
        ),
        (
            Owlrl762RuleId::ScmEqp1Forward,
            vec!["p owl:equivalentProperty q"],
            "p rdfs:subPropertyOf q",
        ),
        (
            Owlrl762RuleId::ScmEqp1Reverse,
            vec!["p owl:equivalentProperty q"],
            "q rdfs:subPropertyOf p",
        ),
        (
            Owlrl762RuleId::ScmSco,
            vec!["C rdfs:subClassOf D", "D rdfs:subClassOf E"],
            "C rdfs:subClassOf E",
        ),
        (
            Owlrl762RuleId::ScmSpo,
            vec!["p rdfs:subPropertyOf q", "q rdfs:subPropertyOf r"],
            "p rdfs:subPropertyOf r",
        ),
        (
            Owlrl762RuleId::ScmEqp2,
            vec!["p rdfs:subPropertyOf q", "q rdfs:subPropertyOf p"],
            "p owl:equivalentProperty q",
        ),
        (
            Owlrl762RuleId::RdfPropertySelfEquivalent,
            vec!["p rdf:type rdf:Property"],
            "p owl:equivalentProperty p",
        ),
    ] {
        assert!(
            Fixture::apply(&reference.definition(), &input).contains(&Fixture::triple(output)),
            "{reference:?}"
        );
    }
}

#[test]
fn reference_identity_and_checked_clauses_survive_both_serde_formats() {
    assert_eq!(Owlrl762RuleId::ALL.len(), 8);
    for id in Owlrl762RuleId::ALL {
        let json = serde_json::to_string(id).unwrap();
        assert!(json.starts_with("\"owlrl-7.6.2/"));
        assert_eq!(serde_json::from_str::<Owlrl762RuleId>(&json).unwrap(), *id);
        assert_eq!(
            postcard::from_bytes::<Owlrl762RuleId>(&postcard::to_allocvec(id).unwrap()).unwrap(),
            *id
        );
        let rule = id.definition();
        assert_eq!(
            serde_json::from_str::<Rule>(&serde_json::to_string(&rule).unwrap()).unwrap(),
            rule
        );
        assert_eq!(
            postcard::from_bytes::<Rule>(&postcard::to_allocvec(&rule).unwrap()).unwrap(),
            rule
        );
    }
    assert!(serde_json::from_str::<Owlrl762RuleId>("\"scm-sco\"").is_err());
    assert!(serde_json::from_str::<Owlrl762RuleId>("\"owlrl-7.6.3/scm-sco\"").is_err());
}
