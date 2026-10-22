//! Independently specified tiny rule fixtures; no production graph evaluator.

mod support;

use rdf_reasoning::RuleId;
use support::Fixture;

#[test]
fn each_rule_has_its_specified_consequence() {
    let fixtures: &[(RuleId, &[&str], &str)] = &[
        (RuleId::Rdf1, &["a p b"], "p rdf:type rdf:Property"),
        (RuleId::Rdfs2, &["p rdfs:domain C", "a p b"], "a rdf:type C"),
        (RuleId::Rdfs3, &["p rdfs:range C", "a p b"], "b rdf:type C"),
        (RuleId::Rdfs4a, &["a p b"], "a rdf:type rdfs:Resource"),
        (
            RuleId::Rdfs4b,
            &["a p literal:value"],
            "literal:value rdf:type rdfs:Resource",
        ),
        (
            RuleId::Rdfs5,
            &["p rdfs:subPropertyOf q", "q rdfs:subPropertyOf r"],
            "p rdfs:subPropertyOf r",
        ),
        (
            RuleId::Rdfs6,
            &["p rdf:type rdf:Property"],
            "p rdfs:subPropertyOf p",
        ),
        (RuleId::Rdfs7, &["p rdfs:subPropertyOf q", "a p b"], "a q b"),
        (
            RuleId::Rdfs8,
            &["C rdf:type rdfs:Class"],
            "C rdfs:subClassOf rdfs:Resource",
        ),
        (
            RuleId::Rdfs9,
            &["a rdf:type C", "C rdfs:subClassOf D"],
            "a rdf:type D",
        ),
        (
            RuleId::Rdfs10,
            &["C rdf:type rdfs:Class"],
            "C rdfs:subClassOf C",
        ),
        (
            RuleId::Rdfs11,
            &["C rdfs:subClassOf D", "D rdfs:subClassOf E"],
            "C rdfs:subClassOf E",
        ),
        (
            RuleId::Rdfs12,
            &["p rdf:type rdfs:ContainerMembershipProperty"],
            "p rdfs:subPropertyOf rdfs:member",
        ),
        (
            RuleId::Rdfs13,
            &["D rdf:type rdfs:Datatype"],
            "D rdfs:subClassOf rdfs:Literal",
        ),
        (
            RuleId::PrpSymp,
            &["p rdf:type owl:SymmetricProperty", "a p b"],
            "b p a",
        ),
        (
            RuleId::PrpTrp,
            &["p rdf:type owl:TransitiveProperty", "a p b", "b p c"],
            "a p c",
        ),
        (RuleId::PrpInv1, &["p owl:inverseOf q", "a p b"], "b q a"),
        (RuleId::PrpInv2, &["p owl:inverseOf q", "a q b"], "b p a"),
    ];
    for (id, premises, conclusion) in fixtures {
        let rule = id.definition();
        let actual = Fixture::apply(&rule, premises);
        assert!(
            actual.contains(&Fixture::triple(conclusion)),
            "{id:?}: {actual:?}"
        );
        assert!(
            Fixture::apply(&rule, &[]).is_empty(),
            "{id:?} inferred without premises"
        );
    }
}

#[test]
fn joins_and_constants_are_not_wildcards() {
    assert!(
        Fixture::apply(
            &RuleId::Rdfs9.definition(),
            &["a rdf:type C", "Other rdfs:subClassOf D"]
        )
        .is_empty()
    );
    assert!(
        Fixture::apply(
            &RuleId::PrpTrp.definition(),
            &["p rdf:type owl:TransitiveProperty", "a p b", "b q c"]
        )
        .is_empty()
    );
    assert!(
        Fixture::apply(
            &RuleId::PrpSymp.definition(),
            &["p rdf:type owl:FunctionalProperty", "a p b"]
        )
        .is_empty()
    );
}
