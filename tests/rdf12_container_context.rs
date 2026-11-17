//! Source container vocabulary cannot be replaced by target-only selection.
#[path = "support/relations.rs"]
mod relations;
use rdf_reasoning::{Rdf12ContainerProperty, RuleId, TRIPLE_RELATION};
use relations::{Fact, closure};

const CONTAINER: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#_937";
const MEMBER: &str = "http://www.w3.org/2000/01/rdf-schema#member";

#[test]
fn source_only_container_predicate_entails_membership_without_target_container_vocabulary() {
    let source = [Fact::new(TRIPLE_RELATION, &["urn:a", CONTAINER, "urn:b"])];
    let target = Fact::new(TRIPLE_RELATION, &["urn:a", MEMBER, "urn:b"]);
    let fallback =
        Rdf12ContainerProperty::try_new("http://www.w3.org/1999/02/22-rdf-syntax-ns#_1").unwrap();
    // bounded: two clauses plus the three seeds for one fallback container.
    let mut rules = vec![RuleId::Rdfs12.definition(), RuleId::Rdfs7.definition()];
    rules.extend(fallback.rdfs_definitions());
    assert!(
        !closure(&rules, &source).contains(&target),
        "target-only fallback misses source semantics"
    );
    let observed = Rdf12ContainerProperty::try_new(CONTAINER).unwrap();
    rules.extend(observed.rdfs_definitions());
    assert!(closure(&rules, &source).contains(&target));
}
