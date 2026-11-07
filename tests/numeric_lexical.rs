//! Exact lexical interpretation and fixed integer boundaries.
use rdf_reasoning::{NumericLexicalError as E, OwlNumericDatatype as D, OwlNumericValue as V};

#[test]
fn decimal_spellings_map_to_exact_values() {
    for (text, coefficient, exponent) in [
        ("+001.2500", "125", -2),
        (".5", "5", -1),
        ("1.", "1", 0),
        ("-0.00", "0", 0),
        (" \t\n-12.5\r ", "-125", -1),
    ] {
        assert_eq!(
            D::Decimal.parse_lexical(text).unwrap(),
            V::try_from_decimal(coefficient, exponent).unwrap()
        );
    }
    assert_eq!(
        D::Integer.parse_lexical(" +0001 ").unwrap(),
        D::Decimal.parse_lexical("1.0").unwrap()
    );
}

#[test]
fn malformed_lexical_forms_fail_without_coercion() {
    for text in [
        "", "+", "-", ".", "+.", "1e2", "NaN", "INF", "1.2.3", "1 2", "1\t2", "١", "１", "\u{a0}1",
        "1\u{b}",
    ] {
        assert_eq!(
            D::Decimal.parse_lexical(text),
            Err(E::InvalidLexicalForm),
            "{text:?}"
        );
    }
    for text in ["1.0", ".5", "1e2", "--1", "+-1"] {
        assert_eq!(D::Integer.parse_lexical(text), Err(E::InvalidLexicalForm));
    }
    for datatype in [D::Float, D::Double, D::Real, D::Rational] {
        assert_eq!(datatype.parse_lexical("1"), Err(E::UnsupportedDatatype));
    }
}

#[test]
fn all_bounded_integer_endpoints_are_checked_exactly() {
    for (datatype, min, max) in [
        (D::Byte, -128i128, 127i128),
        (D::Short, -32768, 32767),
        (D::Int, -2147483648, 2147483647),
        (D::Long, -9223372036854775808, 9223372036854775807),
        (D::UnsignedByte, 0, 255),
        (D::UnsignedShort, 0, 65535),
        (D::UnsignedInt, 0, 4294967295),
        (D::UnsignedLong, 0, 18446744073709551615),
    ] {
        for n in [min, max] {
            assert!(
                datatype.parse_lexical(&n.to_string()).is_ok(),
                "{datatype:?} {n}"
            );
        }
        for n in [min - 1, max + 1] {
            assert_eq!(
                datatype.parse_lexical(&n.to_string()),
                Err(E::OutOfRange),
                "{datatype:?} {n}"
            );
        }
        assert_eq!(datatype.parse_lexical("1.0"), Err(E::InvalidLexicalForm));
        assert!(!V::from_double(1.0).is_member_of(datatype));
        assert!(
            !D::Decimal
                .parse_lexical("0.5")
                .unwrap()
                .is_member_of(datatype)
        );
    }
    assert!(D::UnsignedLong.parse_lexical("-000").is_ok());
}

#[test]
fn sign_restrictions_and_compact_extreme_values_do_not_expand_powers() {
    for (datatype, yes, no) in [
        (D::PositiveInteger, "1", "0"),
        (D::NegativeInteger, "-1", "-0"),
        (D::NonNegativeInteger, "-0", "-1"),
        (D::NonPositiveInteger, "+0", "1"),
    ] {
        assert!(datatype.parse_lexical(yes).is_ok());
        assert_eq!(datatype.parse_lexical(no), Err(E::OutOfRange));
    }
    let huge = V::try_from_decimal("1", i64::MAX).unwrap();
    assert!(huge.is_member_of(D::PositiveInteger));
    assert!(!huge.is_member_of(D::UnsignedLong));
    let tiny = V::try_from_decimal("1", i64::MIN).unwrap();
    assert!(!tiny.is_member_of(D::NonNegativeInteger));
    let long = "9".repeat(1000);
    assert!(
        D::Integer
            .parse_lexical(&long)
            .unwrap()
            .is_member_of(D::PositiveInteger)
    );
    assert_eq!(D::UnsignedLong.parse_lexical(&long), Err(E::OutOfRange));
}

#[test]
fn derived_datatypes_and_errors_round_trip_in_both_formats() {
    for datatype in [
        D::NonPositiveInteger,
        D::NegativeInteger,
        D::Long,
        D::Int,
        D::Short,
        D::Byte,
        D::NonNegativeInteger,
        D::UnsignedLong,
        D::UnsignedInt,
        D::UnsignedShort,
        D::UnsignedByte,
        D::PositiveInteger,
    ] {
        assert_eq!(
            serde_json::from_str::<D>(&serde_json::to_string(&datatype).unwrap()).unwrap(),
            datatype
        );
        assert_eq!(
            postcard::from_bytes::<D>(&postcard::to_allocvec(&datatype).unwrap()).unwrap(),
            datatype
        );
    }
    for error in [
        E::InvalidLexicalForm,
        E::OutOfRange,
        E::UnsupportedDatatype,
        E::RepresentationOverflow,
    ] {
        assert_eq!(
            serde_json::from_str::<E>(&serde_json::to_string(&error).unwrap()).unwrap(),
            error
        );
        assert_eq!(
            postcard::from_bytes::<E>(&postcard::to_allocvec(&error).unwrap()).unwrap(),
            error
        );
    }
    assert_eq!(
        D::UnsignedInt.iri(),
        "http://www.w3.org/2001/XMLSchema#unsignedInt"
    );
}
