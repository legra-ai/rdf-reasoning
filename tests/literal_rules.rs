//! Value-class substitution reuses exact literal identities supplied by adapters.
#[path = "support/relations.rs"]
mod relations;
use rdf_reasoning::{LITERAL_VALUE_RELATION, LiteralRuleId, TRIPLE_RELATION};
use relations::{Fact, closure};

#[test]
fn equal_value_classes_substitute_objects_without_merging_term_identities() {
    let rule = LiteralRuleId::RdfsValueSubstitution.definition();
    let facts = closure(
        &[rule],
        &[
            Fact::new(TRIPLE_RELATION, &["urn:a", "urn:p", "literal-A"]),
            Fact::new(TRIPLE_RELATION, &["urn:b", "urn:q", "literal-B"]),
            Fact::new(TRIPLE_RELATION, &["urn:c", "urn:p", "literal-C"]),
            Fact::new(LITERAL_VALUE_RELATION, &["literal-A", "value-1"]),
            Fact::new(LITERAL_VALUE_RELATION, &["literal-B", "value-1"]),
            Fact::new(LITERAL_VALUE_RELATION, &["literal-C", "value-2"]),
        ],
    );
    assert!(facts.contains(&Fact::new(
        TRIPLE_RELATION,
        &["urn:a", "urn:p", "literal-B"]
    )));
    assert!(facts.contains(&Fact::new(
        TRIPLE_RELATION,
        &["urn:b", "urn:q", "literal-A"]
    )));
    assert!(!facts.contains(&Fact::new(
        TRIPLE_RELATION,
        &["urn:a", "urn:p", "literal-C"]
    )));
    assert!(!facts.contains(&Fact::new(
        TRIPLE_RELATION,
        &["urn:a", "urn:q", "literal-A"]
    )));
}

#[test]
fn value_clause_identifiers_and_checked_rules_round_trip_in_json_and_postcard() {
    for id in LiteralRuleId::ALL {
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
}
