//! Standard finite datatype identity is not the compatibility mathematical key.
use rdf_reasoning::{ExactNumberError, OwlFiniteNumericValue};

#[test]
fn equal_magnitudes_from_disjoint_value_spaces_are_distinct() {
    let decimal = OwlFiniteNumericValue::try_from_decimal("1", 0).unwrap();
    let float = OwlFiniteNumericValue::try_from_float(1.0).unwrap();
    let double = OwlFiniteNumericValue::try_from_double(1.0).unwrap();
    assert_ne!(decimal, float);
    assert_ne!(decimal, double);
    assert_ne!(float, double);
    assert_eq!(
        decimal,
        OwlFiniteNumericValue::try_from_decimal("100", -2).unwrap()
    );
}

#[test]
fn floating_point_signed_zeros_have_distinct_identity() {
    assert_ne!(
        OwlFiniteNumericValue::try_from_float(0.0).unwrap(),
        OwlFiniteNumericValue::try_from_float(-0.0).unwrap()
    );
    assert_ne!(
        OwlFiniteNumericValue::try_from_double(0.0).unwrap(),
        OwlFiniteNumericValue::try_from_double(-0.0).unwrap()
    );
    assert_eq!(
        OwlFiniteNumericValue::try_from_decimal("0", 10).unwrap(),
        OwlFiniteNumericValue::try_from_decimal("-0", -10).unwrap()
    );
}

#[test]
fn nonfinite_values_fail_explicitly_in_this_finite_domain() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            OwlFiniteNumericValue::try_from_double(value),
            Err(ExactNumberError::NonFinite)
        );
    }
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(
            OwlFiniteNumericValue::try_from_float(value),
            Err(ExactNumberError::NonFinite)
        );
    }
}

#[test]
fn values_round_trip_both_formats_and_reject_invalid_wires() {
    for value in [
        OwlFiniteNumericValue::try_from_decimal("10", -1).unwrap(),
        OwlFiniteNumericValue::try_from_float(-0.0).unwrap(),
        OwlFiniteNumericValue::try_from_float(f32::from_bits(1)).unwrap(),
        OwlFiniteNumericValue::try_from_double(-0.0).unwrap(),
        OwlFiniteNumericValue::try_from_double(f64::from_bits(1)).unwrap(),
    ] {
        assert_eq!(
            serde_json::from_str::<OwlFiniteNumericValue>(&serde_json::to_string(&value).unwrap())
                .unwrap(),
            value
        );
        assert_eq!(
            postcard::from_bytes::<OwlFiniteNumericValue>(&postcard::to_allocvec(&value).unwrap())
                .unwrap(),
            value
        );
    }
    for bad in [
        r#"{"float":2139095040}"#,
        r#"{"double":9218868437227405312}"#,
        r#"{"decimal":{"numerator":"1","denominator":"3","power_two":0,"power_five":0}}"#,
        r#"{"unknown":0}"#,
        r#"{"float":0,"extra":1}"#,
    ] {
        assert!(
            serde_json::from_str::<OwlFiniteNumericValue>(bad).is_err(),
            "{bad}"
        );
    }
}

#[test]
fn binary_decoding_rejects_nonfinite_and_nondigital_value_spaces() {
    #[derive(serde::Serialize)]
    enum Wire {
        Decimal(rdf_reasoning::ExactNumber),
        Float(u32),
        Double(u64),
    }
    for wire in [
        Wire::Float(f32::NAN.to_bits()),
        Wire::Double(f64::NEG_INFINITY.to_bits()),
        Wire::Decimal(rdf_reasoning::ExactNumber::try_from_fraction("1", "3").unwrap()),
    ] {
        assert!(
            postcard::from_bytes::<OwlFiniteNumericValue>(&postcard::to_allocvec(&wire).unwrap())
                .is_err()
        );
    }
}
