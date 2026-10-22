//! Exact value keys keep decimal, binary and rational semantics independent.
use rdf_reasoning::{ExactNumber, ExactNumberError};

#[test]
fn equivalent_integer_decimal_binary_and_fraction_keys_are_identical() {
    let one = ExactNumber::try_from_integer("1").unwrap();
    for value in [
        ExactNumber::try_from_integer("+0001").unwrap(),
        ExactNumber::try_from_decimal("100", -2).unwrap(),
        ExactNumber::try_from_f64(1.0).unwrap(),
        ExactNumber::try_from_fraction("-15", "-15").unwrap(),
    ] {
        assert_eq!(value, one);
    }
    assert_eq!(
        ExactNumber::try_from_f64(1.5).unwrap(),
        ExactNumber::try_from_fraction("3", "2").unwrap()
    );
    assert_eq!(
        ExactNumber::try_from_decimal("1", -1).unwrap(),
        ExactNumber::try_from_fraction("1", "10").unwrap()
    );
    assert_eq!(
        ExactNumber::try_from_fraction("2", "12").unwrap(),
        ExactNumber::try_from_fraction("1", "6").unwrap()
    );
    assert_eq!(
        ExactNumber::try_from_f64(-0.0).unwrap(),
        ExactNumber::try_from_fraction("0", "-7").unwrap()
    );
}

#[test]
fn binary_rounding_is_not_decimal_value_equality() {
    let binary = ExactNumber::try_from_f64(0.1).unwrap();
    assert_eq!(
        binary,
        ExactNumber::try_from_fraction("3602879701896397", "36028797018963968").unwrap()
    );
    assert_ne!(binary, ExactNumber::try_from_decimal("1", -1).unwrap());
    assert_ne!(
        ExactNumber::try_from_fraction("1", "3").unwrap(),
        ExactNumber::try_from_decimal("333333", -6).unwrap()
    );
    let smallest = ExactNumber::try_from_f64(f64::from_bits(1)).unwrap();
    assert_eq!(smallest.to_string(), "1/1*2^-1074*5^0");
}

#[test]
fn huge_exponents_stay_compact_and_invalid_inputs_fail() {
    let value = ExactNumber::try_from_decimal("1", 1_000_000_000).unwrap();
    assert!(postcard::to_allocvec(&value).unwrap().len() < 64);
    assert_eq!(value.to_string(), "1/1*2^1000000000*5^1000000000");
    assert_eq!(
        ExactNumber::try_from_fraction("1", "0"),
        Err(ExactNumberError::ZeroDenominator)
    );
    assert_eq!(
        ExactNumber::try_from_integer("1_0"),
        Err(ExactNumberError::InvalidInteger)
    );
    assert_eq!(
        ExactNumber::try_from_decimal("10", i64::MAX),
        Err(ExactNumberError::ExponentOverflow)
    );
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            ExactNumber::try_from_f64(value),
            Err(ExactNumberError::NonFinite)
        );
    }
}

#[test]
fn numeric_keys_round_trip_and_validate_both_serde_formats() {
    #[derive(serde::Serialize)]
    struct Wire<'a> {
        numerator: &'a str,
        denominator: &'a str,
        power_two: i64,
        power_five: i64,
    }
    for value in [
        ExactNumber::try_from_fraction("-45", "14").unwrap(),
        ExactNumber::try_from_decimal("25", -2).unwrap(),
    ] {
        let json = serde_json::to_string(&value).unwrap();
        let bytes = postcard::to_allocvec(&value).unwrap();
        assert_eq!(serde_json::from_str::<ExactNumber>(&json).unwrap(), value);
        assert_eq!(postcard::from_bytes::<ExactNumber>(&bytes).unwrap(), value);
    }
    for value in [
        Wire {
            numerator: "1",
            denominator: "0",
            power_two: 0,
            power_five: 0,
        },
        Wire {
            numerator: "x",
            denominator: "1",
            power_two: 0,
            power_five: 0,
        },
        Wire {
            numerator: "10",
            denominator: "1",
            power_two: i64::MAX,
            power_five: 0,
        },
    ] {
        assert!(
            serde_json::from_str::<ExactNumber>(&serde_json::to_string(&value).unwrap()).is_err()
        );
        assert!(
            postcard::from_bytes::<ExactNumber>(&postcard::to_allocvec(&value).unwrap()).is_err()
        );
    }
}
