//! Independent fixtures for numeric identity and restricted datatype membership.
use rdf_reasoning::{OwlNumericDatatype as D, OwlNumericValue as V};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash(value: &V) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn nan_payloads_have_one_identity_per_disjoint_space() {
    let float = V::from_float(f32::NAN);
    let double = V::from_double(f64::NAN);
    for bits in [0x7f800001, 0x7fc00000, 0xffc01234] {
        let other = V::from_float(f32::from_bits(bits));
        assert_eq!(float, other);
        assert_eq!(hash(&float), hash(&other));
    }
    for bits in [0x7ff0000000000001, 0x7ff8000000000000, 0xfff8000000001234] {
        let other = V::from_double(f64::from_bits(bits));
        assert_eq!(double, other);
        assert_eq!(hash(&double), hash(&other));
    }
    assert_ne!(float, double);
}

#[test]
fn special_values_and_zeros_remain_distinct() {
    let values = [
        V::from_float(f32::NEG_INFINITY),
        V::from_float(-0.0),
        V::from_float(0.0),
        V::from_float(f32::INFINITY),
        V::from_float(f32::NAN),
        V::from_double(f64::NEG_INFINITY),
        V::from_double(-0.0),
        V::from_double(0.0),
        V::from_double(f64::INFINITY),
        V::from_double(f64::NAN),
        V::try_from_decimal("0", 0).unwrap(),
    ];
    for (i, left) in values.iter().enumerate() {
        for (j, right) in values.iter().enumerate() {
            assert_eq!(left == right, i == j);
        }
    }
}

#[test]
fn membership_uses_value_spaces_not_arithmetic_conversion() {
    let datatypes = [
        D::Decimal,
        D::Integer,
        D::Rational,
        D::Real,
        D::Float,
        D::Double,
    ];
    for (value, expected) in [
        (
            V::try_from_decimal("100", -2).unwrap(),
            [true, true, true, true, false, false],
        ),
        (
            V::try_from_decimal("1", -1).unwrap(),
            [true, false, true, true, false, false],
        ),
        (
            V::try_from_decimal("0", i64::MIN).unwrap(),
            [true, true, true, true, false, false],
        ),
        (
            V::try_from_decimal("1", i64::MAX).unwrap(),
            [true, true, true, true, false, false],
        ),
        (
            V::try_from_decimal("1", i64::MIN).unwrap(),
            [true, false, true, true, false, false],
        ),
        (
            V::from_float(1.0),
            [false, false, false, false, true, false],
        ),
        (
            V::from_float(f32::NAN),
            [false, false, false, false, true, false],
        ),
        (
            V::from_double(f64::INFINITY),
            [false, false, false, false, false, true],
        ),
    ] {
        for (datatype, member) in datatypes.into_iter().zip(expected) {
            assert_eq!(
                value.is_member_of(datatype),
                member,
                "{value:?} {datatype:?}"
            );
        }
    }
}

#[test]
fn nested_transport_preserves_all_special_values() {
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Fixture {
        datatype: D,
        value: V,
    }
    for value in [
        V::from_float(f32::NAN),
        V::from_float(f32::INFINITY),
        V::from_float(f32::NEG_INFINITY),
        V::from_float(-0.0),
        V::from_double(f64::NAN),
        V::from_double(f64::INFINITY),
        V::from_double(f64::NEG_INFINITY),
        V::from_double(-0.0),
        V::from_double(f64::from_bits(1)),
        V::from_float(f32::from_bits(1)),
        V::try_from_decimal("125", -2).unwrap(),
    ] {
        for datatype in [
            D::Decimal,
            D::Integer,
            D::Rational,
            D::Real,
            D::Float,
            D::Double,
        ] {
            let fixture = Fixture {
                datatype,
                value: value.clone(),
            };
            assert_eq!(
                serde_json::from_str::<Fixture>(&serde_json::to_string(&fixture).unwrap()).unwrap(),
                fixture
            );
            assert_eq!(
                postcard::from_bytes::<Fixture>(&postcard::to_allocvec(&fixture).unwrap()).unwrap(),
                fixture
            );
        }
    }
}

#[test]
fn checked_decoders_reject_noncanonical_or_unknown_values() {
    #[derive(serde::Serialize)]
    enum Wire {
        Decimal(rdf_reasoning::ExactNumber),
        Float(u32),
        Double(u64),
    }
    for wire in [
        Wire::Float(0x7f800001),
        Wire::Double(0xfff8000000000001),
        Wire::Decimal(rdf_reasoning::ExactNumber::try_from_fraction("1", "3").unwrap()),
    ] {
        assert!(postcard::from_bytes::<V>(&postcard::to_allocvec(&wire).unwrap()).is_err());
    }
    for text in [
        r#"{"float":2139095041}"#,
        r#"{"double":18444492273895866369}"#,
        r#"{"unknown":0}"#,
        r#"{"float":0,"extra":1}"#,
        r#"{"decimal":{"numerator":"1","denominator":"3","power_two":0,"power_five":0}}"#,
    ] {
        assert!(serde_json::from_str::<V>(text).is_err(), "{text}");
    }
    assert!(serde_json::from_str::<D>("\"unsigned-int\"").is_err());
    assert!(postcard::from_bytes::<D>(&[6]).is_err());
}
