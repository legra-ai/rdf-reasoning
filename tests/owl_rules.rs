//! Independently specified positive OWL consequences and generalized term identity.

mod support;

use datalog_structural_model::{DatalogProfile, Program, Rule};
use rdf_reasoning::{RuleId, RuleId::*};
use support::Fixture;

#[test]
fn each_added_owl_clause_has_its_specified_consequence() {
    let fixtures: &[(RuleId, &[&str], &str)] = &[
        (EqRefSubject, &["a p b"], "a owl:sameAs a"),
        (EqRefPredicate, &["a p b"], "p owl:sameAs p"),
        (
            EqRefObject,
            &["a p literal:value"],
            "literal:value owl:sameAs literal:value",
        ),
        (EqSym, &["a owl:sameAs b"], "b owl:sameAs a"),
        (
            EqTrans,
            &["a owl:sameAs b", "b owl:sameAs c"],
            "a owl:sameAs c",
        ),
        (
            EqRepS,
            &["a owl:sameAs literal:value", "a p b"],
            "literal:value p b",
        ),
        (EqRepP, &["p owl:sameAs q", "a p b"], "a q b"),
        (EqRepO, &["a owl:sameAs b", "x p a"], "x p b"),
        (
            PrpFp,
            &["p rdf:type owl:FunctionalProperty", "a p b", "a p c"],
            "b owl:sameAs c",
        ),
        (
            PrpIfp,
            &["p rdf:type owl:InverseFunctionalProperty", "a p c", "b p c"],
            "a owl:sameAs b",
        ),
        (PrpEqp1, &["p owl:equivalentProperty q", "a p b"], "a q b"),
        (PrpEqp2, &["p owl:equivalentProperty q", "a q b"], "a p b"),
        (
            ClsSvf1,
            &[
                "R owl:onProperty p",
                "R owl:someValuesFrom C",
                "a p b",
                "b rdf:type C",
            ],
            "a rdf:type R",
        ),
        (
            ClsSvf2,
            &[
                "R owl:onProperty p",
                "R owl:someValuesFrom owl:Thing",
                "a p literal:value",
            ],
            "a rdf:type R",
        ),
        (
            ClsAvf,
            &[
                "R owl:onProperty p",
                "R owl:allValuesFrom C",
                "a rdf:type R",
                "a p b",
            ],
            "b rdf:type C",
        ),
        (
            ClsHv1,
            &[
                "R owl:onProperty p",
                "R owl:hasValue literal:value",
                "a rdf:type R",
            ],
            "a p literal:value",
        ),
        (
            ClsHv2,
            &[
                "R owl:onProperty p",
                "R owl:hasValue literal:value",
                "a p literal:value",
            ],
            "a rdf:type R",
        ),
        (
            CaxEqc1,
            &["C owl:equivalentClass D", "a rdf:type C"],
            "a rdf:type D",
        ),
        (
            CaxEqc2,
            &["C owl:equivalentClass D", "a rdf:type D"],
            "a rdf:type C",
        ),
        (
            ScmClsSelfSubclass,
            &["C rdf:type owl:Class"],
            "C rdfs:subClassOf C",
        ),
        (
            ScmClsSelfEquivalent,
            &["C rdf:type owl:Class"],
            "C owl:equivalentClass C",
        ),
        (
            ScmClsThing,
            &["C rdf:type owl:Class"],
            "C rdfs:subClassOf owl:Thing",
        ),
        (
            ScmClsNothing,
            &["C rdf:type owl:Class"],
            "owl:Nothing rdfs:subClassOf C",
        ),
        (
            ScmEqc1Forward,
            &["C owl:equivalentClass D"],
            "C rdfs:subClassOf D",
        ),
        (
            ScmEqc1Reverse,
            &["C owl:equivalentClass D"],
            "D rdfs:subClassOf C",
        ),
        (
            ScmEqc2,
            &["C rdfs:subClassOf D", "D rdfs:subClassOf C"],
            "C owl:equivalentClass D",
        ),
        (
            ScmEqp1Forward,
            &["p owl:equivalentProperty q"],
            "p rdfs:subPropertyOf q",
        ),
        (
            ScmEqp1Reverse,
            &["p owl:equivalentProperty q"],
            "q rdfs:subPropertyOf p",
        ),
        (
            ScmEqp2,
            &["p rdfs:subPropertyOf q", "q rdfs:subPropertyOf p"],
            "p owl:equivalentProperty q",
        ),
        (
            ScmDom1,
            &["p rdfs:domain C", "C rdfs:subClassOf D"],
            "p rdfs:domain D",
        ),
        (
            ScmDom2,
            &["q rdfs:domain C", "p rdfs:subPropertyOf q"],
            "p rdfs:domain C",
        ),
        (
            ScmRng1,
            &["p rdfs:range C", "C rdfs:subClassOf D"],
            "p rdfs:range D",
        ),
        (
            ScmRng2,
            &["q rdfs:range C", "p rdfs:subPropertyOf q"],
            "p rdfs:range C",
        ),
        (
            ScmHv,
            &[
                "R owl:hasValue literal:value",
                "R owl:onProperty p",
                "S owl:hasValue literal:value",
                "S owl:onProperty q",
                "p rdfs:subPropertyOf q",
            ],
            "R rdfs:subClassOf S",
        ),
        (
            ScmSvf1,
            &[
                "R owl:someValuesFrom C",
                "R owl:onProperty p",
                "S owl:someValuesFrom D",
                "S owl:onProperty p",
                "C rdfs:subClassOf D",
            ],
            "R rdfs:subClassOf S",
        ),
        (
            ScmSvf2,
            &[
                "R owl:someValuesFrom C",
                "R owl:onProperty p",
                "S owl:someValuesFrom C",
                "S owl:onProperty q",
                "p rdfs:subPropertyOf q",
            ],
            "R rdfs:subClassOf S",
        ),
        (
            ScmAvf1,
            &[
                "R owl:allValuesFrom C",
                "R owl:onProperty p",
                "S owl:allValuesFrom D",
                "S owl:onProperty p",
                "C rdfs:subClassOf D",
            ],
            "R rdfs:subClassOf S",
        ),
        (
            ScmAvf2,
            &[
                "R owl:allValuesFrom C",
                "R owl:onProperty p",
                "S owl:allValuesFrom C",
                "S owl:onProperty q",
                "p rdfs:subPropertyOf q",
            ],
            "S rdfs:subClassOf R",
        ),
        (
            ScmOpSelfSubproperty,
            &["p rdf:type owl:ObjectProperty"],
            "p rdfs:subPropertyOf p",
        ),
        (
            ScmOpSelfEquivalent,
            &["p rdf:type owl:ObjectProperty"],
            "p owl:equivalentProperty p",
        ),
        (
            ScmDpSelfSubproperty,
            &["p rdf:type owl:DatatypeProperty"],
            "p rdfs:subPropertyOf p",
        ),
        (
            ScmDpSelfEquivalent,
            &["p rdf:type owl:DatatypeProperty"],
            "p owl:equivalentProperty p",
        ),
    ];
    assert_eq!(fixtures.len(), 42);
    for (id, premises, conclusion) in fixtures {
        let rule = id.definition();
        assert!(
            Fixture::apply(&rule, premises).contains(&Fixture::triple(conclusion)),
            "{id:?}"
        );
        assert!(
            Fixture::apply(&rule, &[]).is_empty(),
            "{id:?} inferred without premises"
        );
    }
}

#[test]
fn added_owl_joins_do_not_cross_subjects_properties_or_fillers() {
    for (id, input) in [
        (
            RuleId::PrpFp,
            vec!["p rdf:type owl:FunctionalProperty", "a p b", "c q d"],
        ),
        (
            RuleId::ClsSvf1,
            vec![
                "R owl:onProperty p",
                "R owl:someValuesFrom C",
                "a p b",
                "other rdf:type C",
            ],
        ),
        (
            RuleId::ScmHv,
            vec![
                "R owl:hasValue a",
                "R owl:onProperty p",
                "S owl:hasValue b",
                "S owl:onProperty q",
                "p rdfs:subPropertyOf q",
            ],
        ),
    ] {
        let results = Fixture::apply(&id.definition(), &input);
        if id == RuleId::PrpFp {
            assert!(!results.contains(&Fixture::triple("b owl:sameAs d")));
        } else {
            assert!(results.is_empty(), "{id:?}");
        }
    }
}

#[test]
fn complete_catalogue_is_positive_and_round_trips_in_text_and_binary() {
    assert_eq!(RuleId::ALL.len(), 60);
    for id in RuleId::ALL {
        let rule = id.definition();
        Program::try_new(vec![rule.clone()])
            .unwrap()
            .validate_for(&DatalogProfile::positive())
            .unwrap();
        let json = serde_json::to_string(&rule).unwrap();
        assert_eq!(serde_json::from_str::<Rule>(&json).unwrap(), rule);
        assert_eq!(
            postcard::from_bytes::<Rule>(&postcard::to_allocvec(&rule).unwrap()).unwrap(),
            rule
        );
        assert_eq!(
            serde_json::from_str::<RuleId>(&serde_json::to_string(id).unwrap()).unwrap(),
            *id
        );
        assert_eq!(
            postcard::from_bytes::<RuleId>(&postcard::to_allocvec(id).unwrap()).unwrap(),
            *id
        );
    }
}
