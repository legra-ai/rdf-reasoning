//! Numeric fixtures distinguish datatype identity from arithmetic and RDF identity.
#[path = "support/relations.rs"]
mod relations;

use datalog_structural_model::{BuiltinFilter, Literal, Rule};
use rdf_reasoning::{
    OWL_NUMERIC_VALUE_RELATION, OwlNumericRuleId, OwlNumericValue as V, TRIPLE_RELATION,
};
use relations::{Fact, closure};

const SAME: &str = "http://www.w3.org/2002/07/owl#sameAs";
const DIFFERENT: &str = "http://www.w3.org/2002/07/owl#differentFrom";

#[test]
fn numeric_clauses_preserve_literal_identity_and_all_ordered_pairs() {
    let cases = [
        (
            V::try_from_decimal("1", 0).unwrap(),
            V::try_from_decimal("100", -2).unwrap(),
            true,
        ),
        (
            V::try_from_decimal("1", 0).unwrap(),
            V::from_float(1.0),
            false,
        ),
        (V::from_float(1.0), V::from_double(1.0), false),
        (V::from_float(0.0), V::from_float(-0.0), false),
        (V::from_double(0.0), V::from_double(-0.0), false),
        (
            V::try_from_decimal("0", 0).unwrap(),
            V::try_from_decimal("-0", 0).unwrap(),
            true,
        ),
        (
            V::from_float(f32::INFINITY),
            V::from_float(f32::INFINITY),
            true,
        ),
        (
            V::from_float(f32::INFINITY),
            V::from_float(f32::NEG_INFINITY),
            false,
        ),
        (
            V::from_double(f64::NEG_INFINITY),
            V::from_double(f64::NEG_INFINITY),
            true,
        ),
        (
            V::from_float(f32::INFINITY),
            V::from_double(f64::INFINITY),
            false,
        ),
        (
            V::from_float(f32::NAN),
            V::from_float(f32::from_bits(0xff800001)),
            true,
        ),
        (
            V::from_double(f64::NAN),
            V::from_double(f64::from_bits(0xfff0000000000001)),
            true,
        ),
        (V::from_float(f32::NAN), V::from_double(f64::NAN), false),
        (V::from_double(f64::NAN), V::from_double(0.0), false),
        (
            V::try_from_decimal("1", -1).unwrap(),
            V::from_double(0.1),
            false,
        ),
    ];
    let rules = OwlNumericRuleId::ALL
        .iter()
        .map(|id| id.definition())
        .collect::<Vec<_>>();
    for (left, right, equal) in cases {
        // bounded: two interpreted fixture literals; keys use the public value identity.
        let right_key = if left == right {
            "key-left"
        } else {
            "key-right"
        };
        let facts = closure(
            &rules,
            &[
                Fact::new(OWL_NUMERIC_VALUE_RELATION, &["literal-left", "key-left"]),
                Fact::new(OWL_NUMERIC_VALUE_RELATION, &["literal-right", right_key]),
            ],
        );
        for (a, b) in [
            ("literal-left", "literal-right"),
            ("literal-right", "literal-left"),
        ] {
            assert_eq!(
                facts.contains(&Fact::new(TRIPLE_RELATION, &[a, SAME, b])),
                equal,
                "{left:?} {right:?}"
            );
            assert_eq!(
                facts.contains(&Fact::new(TRIPLE_RELATION, &[a, DIFFERENT, b])),
                !equal,
                "{left:?} {right:?}"
            );
        }
        for literal in ["literal-left", "literal-right"] {
            assert!(facts.contains(&Fact::new(TRIPLE_RELATION, &[literal, SAME, literal])));
            assert!(!facts.contains(&Fact::new(TRIPLE_RELATION, &[literal, DIFFERENT, literal])));
        }
        assert_eq!(facts.len(), 6);
    }
}

#[test]
fn numeric_domain_is_not_inferred_from_arbitrary_triples_or_compatibility_keys() {
    let rules = OwlNumericRuleId::ALL
        .iter()
        .map(|id| id.definition())
        .collect::<Vec<_>>();
    let input = [
        Fact::new(TRIPLE_RELATION, &["urn:s", "urn:p", "unknown-literal"]),
        Fact::new(
            rdf_reasoning::LITERAL_VALUE_RELATION,
            &["compatibility-literal", "key"],
        ),
        Fact::new(OWL_NUMERIC_VALUE_RELATION, &["numeric-literal", "key"]),
    ];
    let facts = closure(&rules, &input);
    assert_eq!(facts.len(), input.len() + 1);
    assert!(facts.contains(&Fact::new(
        TRIPLE_RELATION,
        &["numeric-literal", SAME, "numeric-literal"]
    )));
    assert_eq!(closure(&rules, &[]).len(), 0);
}

#[test]
fn clauses_and_exact_restricted_identifiers_round_trip() {
    assert_eq!(
        OwlNumericRuleId::ALL,
        &[OwlNumericRuleId::DtEq, OwlNumericRuleId::DtDiff]
    );
    for (id, name) in [
        (OwlNumericRuleId::DtEq, "numeric/dt-eq"),
        (OwlNumericRuleId::DtDiff, "numeric/dt-diff"),
    ] {
        assert_eq!(serde_json::to_string(&id).unwrap(), format!("\"{name}\""));
        assert_eq!(
            serde_json::from_str::<OwlNumericRuleId>(&format!("\"{name}\"")).unwrap(),
            id
        );
        assert_eq!(
            postcard::from_bytes::<OwlNumericRuleId>(&postcard::to_allocvec(&id).unwrap()).unwrap(),
            id
        );
        let rule = id.definition();
        assert!(rule.is_safe());
        assert_eq!(
            serde_json::from_str::<Rule>(&serde_json::to_string(&rule).unwrap()).unwrap(),
            rule
        );
        assert_eq!(
            postcard::from_bytes::<Rule>(&postcard::to_allocvec(&rule).unwrap()).unwrap(),
            rule
        );
        assert!(rule.body.iter().all(|literal| matches!(
            literal,
            Literal::Positive(_) | Literal::Filter(BuiltinFilter::Ne(_, _))
        )));
    }
    for bad in ["dt-eq", "dt-diff", "numeric/dt-type2", "owl2-rl"] {
        assert!(serde_json::from_str::<OwlNumericRuleId>(&format!("\"{bad}\"")).is_err());
    }
    assert!(postcard::from_bytes::<OwlNumericRuleId>(&[2]).is_err());
}
