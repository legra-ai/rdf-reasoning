//! Independent finite fixtures for positive RDF/OWL list consequences.
#[path = "support/relations.rs"]
mod relations;
use rdf_reasoning::{ListRuleId, RDF_LIST_MEMBER_RELATION, TRIPLE_RELATION};
use relations::{Fact, closure};

const FIRST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#first";
const REST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#rest";
const TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const SUB: &str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";

#[test]
fn list_membership_reaches_only_its_own_cells_and_terminates_on_cycles() {
    let rules = [
        ListRuleId::MemberFirst.definition(),
        ListRuleId::MemberRest.definition(),
    ];
    let facts = closure(
        &rules,
        &[
            Fact::new(TRIPLE_RELATION, &["urn:l", FIRST, "urn:A"]),
            Fact::new(TRIPLE_RELATION, &["urn:l", REST, "urn:tail"]),
            Fact::new(TRIPLE_RELATION, &["urn:tail", FIRST, "urn:B"]),
            Fact::new(TRIPLE_RELATION, &["urn:tail", REST, "urn:l"]),
            Fact::new(TRIPLE_RELATION, &["urn:other", FIRST, "urn:C"]),
        ],
    );
    assert!(facts.contains(&Fact::new(RDF_LIST_MEMBER_RELATION, &["urn:l", "urn:A"])));
    assert!(facts.contains(&Fact::new(RDF_LIST_MEMBER_RELATION, &["urn:l", "urn:B"])));
    assert!(!facts.contains(&Fact::new(RDF_LIST_MEMBER_RELATION, &["urn:l", "urn:C"])));
    assert!(!facts.contains(&Fact::new(
        RDF_LIST_MEMBER_RELATION,
        &["urn:empty", "urn:A"]
    )));
}

#[test]
fn every_list_consequence_joins_the_declared_list() {
    for (id, predicate, membership, expected) in [
        (
            ListRuleId::ClsInt2,
            "intersectionOf",
            ["urn:x", TYPE, "urn:C"],
            ["urn:x", TYPE, "urn:A"],
        ),
        (
            ListRuleId::ClsUni,
            "unionOf",
            ["urn:x", TYPE, "urn:A"],
            ["urn:x", TYPE, "urn:C"],
        ),
        (
            ListRuleId::ClsOo,
            "oneOf",
            ["urn:irrelevant", TYPE, "urn:C"],
            ["urn:A", TYPE, "urn:C"],
        ),
        (
            ListRuleId::ScmInt,
            "intersectionOf",
            ["urn:irrelevant", TYPE, "urn:C"],
            ["urn:C", SUB, "urn:A"],
        ),
        (
            ListRuleId::ScmUni,
            "unionOf",
            ["urn:irrelevant", TYPE, "urn:C"],
            ["urn:A", SUB, "urn:C"],
        ),
    ] {
        let predicate = format!("http://www.w3.org/2002/07/owl#{predicate}");
        let rules = ListRuleId::ALL
            .iter()
            .map(|id| id.definition())
            .collect::<Vec<_>>();
        let input = [
            Fact::new(TRIPLE_RELATION, &["urn:C", &predicate, "urn:l"]),
            Fact::new(TRIPLE_RELATION, &["urn:l", FIRST, "urn:A"]),
            Fact::new(TRIPLE_RELATION, &membership),
            Fact::new(TRIPLE_RELATION, &["urn:other", FIRST, "urn:unrelated"]),
        ];
        let result = closure(&rules, &input);
        assert!(
            result.contains(&Fact::new(TRIPLE_RELATION, &expected)),
            "{id:?}"
        );
        assert!(!result.contains(&Fact::new(
            RDF_LIST_MEMBER_RELATION,
            &["urn:l", "urn:unrelated"]
        )));
        let mut mismatched = input.clone();
        mismatched[0] = Fact::new(TRIPLE_RELATION, &["urn:C", &predicate, "urn:other"]);
        assert!(
            !closure(&rules, &mismatched).contains(&Fact::new(TRIPLE_RELATION, &expected)),
            "cross-list join: {id:?}"
        );
    }
}

#[test]
fn list_clauses_round_trip_checked_rules_in_text_and_binary() {
    for id in ListRuleId::ALL {
        let rule = id.definition();
        assert!(rule.is_safe());
        assert_eq!(
            *id,
            serde_json::from_str(&serde_json::to_string(id).unwrap()).unwrap()
        );
        assert_eq!(
            *id,
            postcard::from_bytes(&postcard::to_allocvec(id).unwrap()).unwrap()
        );
        assert_eq!(
            rule,
            serde_json::from_str(&serde_json::to_string(&rule).unwrap()).unwrap()
        );
        assert_eq!(
            rule,
            postcard::from_bytes(&postcard::to_allocvec(&rule).unwrap()).unwrap()
        );
    }
    assert!(serde_json::from_str::<ListRuleId>("\"full-owlrl-list-rules\"").is_err());
}
