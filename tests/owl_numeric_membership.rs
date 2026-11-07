//! Numeric membership evidence is distinct from asserted types and violations.
#[path = "support/relations.rs"]
mod relations;

use rdf_reasoning::{
    OWL_NUMERIC_MEMBER_RELATION as MEMBER, OWL_NUMERIC_NONMEMBER_RELATION as NONMEMBER,
    OWL_NUMERIC_TYPE_VIOLATION_RELATION as VIOLATION, OwlNumericDatatype as D,
    OwlNumericMembershipRuleId as R, OwlNumericTypeViolation as Violation, OwlNumericValue as V,
    TRIPLE_RELATION,
};
use relations::{Fact, closure};

const TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

#[test]
fn positive_membership_produces_only_supported_numeric_types() {
    let value = V::try_from_decimal("10", -1).unwrap();
    for datatype in [
        D::Decimal,
        D::Integer,
        D::Rational,
        D::Real,
        D::Float,
        D::Double,
    ] {
        let member = value.is_member_of(datatype);
        let relation = if member { MEMBER } else { NONMEMBER };
        let result = closure(
            &[R::DtType2.definition()],
            &[Fact::new(relation, &["literal", datatype.iri()])],
        );
        assert_eq!(
            result.contains(&Fact::new(
                TRIPLE_RELATION,
                &["literal", TYPE, datatype.iri()]
            )),
            member
        );
        assert_eq!(result.len(), 1 + usize::from(member));
    }
}

#[test]
fn contradictions_require_assertion_and_matching_nonmembership_evidence() {
    let rules = [R::DtType2.definition(), R::DtNotType.definition()];
    let facts = closure(
        &rules,
        &[
            Fact::new(TRIPLE_RELATION, &["bad", TYPE, D::Integer.iri()]),
            Fact::new(NONMEMBER, &["bad", D::Integer.iri()]),
            Fact::new(NONMEMBER, &["not-asserted", D::Integer.iri()]),
            Fact::new(TRIPLE_RELATION, &["unknown", TYPE, D::Integer.iri()]),
            Fact::new(
                TRIPLE_RELATION,
                &["different-datatype", TYPE, D::Float.iri()],
            ),
            Fact::new(NONMEMBER, &["different-datatype", D::Double.iri()]),
            Fact::new(MEMBER, &["good", D::Integer.iri()]),
        ],
    );
    assert!(facts.contains(&Fact::new(VIOLATION, &["bad", D::Integer.iri()])));
    assert_eq!(facts.iter().filter(|fact| fact.0 == VIOLATION).count(), 1);
    assert!(facts.contains(&Fact::new(
        TRIPLE_RELATION,
        &["good", TYPE, D::Integer.iri()]
    )));
    assert_eq!(facts.len(), 9);
    assert_eq!(closure(&rules, &[]).len(), 0);
}

#[test]
fn typed_checks_cover_special_values_and_preserve_caller_literal_identity() {
    for (value, datatype, member) in [
        (V::try_from_decimal("1", -1).unwrap(), D::Integer, false),
        (
            V::try_from_decimal("1", i64::MAX).unwrap(),
            D::Integer,
            true,
        ),
        (V::from_float(-0.0), D::Float, true),
        (V::from_float(-0.0), D::Decimal, false),
        (V::from_float(f32::NAN), D::Float, true),
        (V::from_double(f64::NAN), D::Float, false),
        (V::from_double(f64::INFINITY), D::Double, true),
        (V::from_double(f64::NEG_INFINITY), D::Real, false),
    ] {
        let result = Violation::check_assertion("exact-literal", value.clone(), datatype);
        assert_eq!(result.is_ok(), member);
        if let Err(violation) = result {
            assert_eq!(*violation.literal(), "exact-literal");
            assert_eq!(violation.value(), &value);
            assert_eq!(violation.datatype(), datatype);
            assert_eq!(
                serde_json::from_str::<Violation<String>>(
                    &serde_json::to_string(&violation).unwrap()
                )
                .unwrap()
                .literal(),
                "exact-literal"
            );
        }
    }
}

#[test]
fn nested_diagnostics_round_trip_and_forged_membership_is_rejected() {
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Report {
        violation: Violation<String>,
    }
    let value = V::from_double(f64::NAN);
    let report = Report {
        violation: Violation::check_assertion("literal".to_owned(), value, D::Integer).unwrap_err(),
    };
    assert_eq!(
        serde_json::from_str::<Report>(&serde_json::to_string(&report).unwrap()).unwrap(),
        report
    );
    assert_eq!(
        postcard::from_bytes::<Report>(&postcard::to_allocvec(&report).unwrap()).unwrap(),
        report
    );

    #[derive(serde::Serialize)]
    struct Wire {
        literal: String,
        value: V,
        datatype: D,
    }
    let forged = Wire {
        literal: "literal".to_owned(),
        value: V::from_float(f32::NAN),
        datatype: D::Float,
    };
    assert!(
        serde_json::from_str::<Violation<String>>(&serde_json::to_string(&forged).unwrap())
            .is_err()
    );
    assert!(
        postcard::from_bytes::<Violation<String>>(&postcard::to_allocvec(&forged).unwrap())
            .is_err()
    );
    for invalid in [
        r#"{"literal":"x","value":{"float":0},"datatype":"double","extra":1}"#,
        r#"{"literal":"x","value":{"float":0},"datatype":"unknown"}"#,
        r#"{"literal":"x","value":{"float":2139095041},"datatype":"double"}"#,
    ] {
        assert!(serde_json::from_str::<Violation<String>>(invalid).is_err());
    }
}

#[test]
fn membership_clauses_have_exact_names_and_checked_transport() {
    assert_eq!(R::ALL, &[R::DtType2, R::DtNotType]);
    for (id, name) in [
        (R::DtType2, "numeric/dt-type2"),
        (R::DtNotType, "numeric/dt-not-type"),
    ] {
        let rule = id.definition();
        assert!(rule.is_safe());
        assert_eq!(serde_json::to_string(&id).unwrap(), format!("\"{name}\""));
        assert_eq!(
            serde_json::from_str::<R>(&serde_json::to_string(&id).unwrap()).unwrap(),
            id
        );
        assert_eq!(
            postcard::from_bytes::<R>(&postcard::to_allocvec(&id).unwrap()).unwrap(),
            id
        );
        assert_eq!(
            serde_json::from_str::<datalog_structural_model::Rule>(
                &serde_json::to_string(&rule).unwrap()
            )
            .unwrap(),
            rule
        );
        assert_eq!(
            postcard::from_bytes::<datalog_structural_model::Rule>(
                &postcard::to_allocvec(&rule).unwrap()
            )
            .unwrap(),
            rule
        );
    }
    for name in ["dt-type2", "dt-not-type", "numeric/dt-type1"] {
        assert!(serde_json::from_str::<R>(&format!("\"{name}\"")).is_err());
    }
    assert!(postcard::from_bytes::<R>(&[2]).is_err());
}
