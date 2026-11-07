//! Independent IEEE bit fixtures for shared XML Schema floating lexical mapping.
use num_bigint::BigUint;
use rdf_reasoning::{NumericLexicalError as E, OwlNumericDatatype as D, OwlNumericValue as V};

#[test]
fn valid_grammar_preserves_spaces_specials_and_signed_zero() {
    for text in ["1", "+1.0", ".1e1", "1.E+0", " \t1e0\r\n"] {
        assert_eq!(D::Float.parse_lexical(text).unwrap(), V::from_float(1.0));
        assert_eq!(D::Double.parse_lexical(text).unwrap(), V::from_double(1.0));
    }
    for text in ["INF", "+INF", " \n+INF\t"] {
        assert_eq!(
            D::Float.parse_lexical(text).unwrap(),
            V::from_float(f32::INFINITY)
        );
        assert_eq!(
            D::Double.parse_lexical(text).unwrap(),
            V::from_double(f64::INFINITY)
        );
    }
    assert_eq!(
        D::Float.parse_lexical("-INF").unwrap(),
        V::from_float(f32::NEG_INFINITY)
    );
    assert_eq!(
        D::Double.parse_lexical("-INF").unwrap(),
        V::from_double(f64::NEG_INFINITY)
    );
    for text in ["-0", "-0.0", "-0e999999", "-1e-999999"] {
        assert_eq!(D::Float.parse_lexical(text).unwrap(), V::from_float(-0.0));
        assert_eq!(D::Double.parse_lexical(text).unwrap(), V::from_double(-0.0));
    }
    assert_eq!(
        D::Float.parse_lexical("NaN").unwrap(),
        V::from_float(f32::NAN)
    );
    assert_eq!(
        D::Double.parse_lexical("NaN").unwrap(),
        V::from_double(f64::NAN)
    );
    assert_ne!(
        D::Float.parse_lexical("1").unwrap(),
        D::Double.parse_lexical("1").unwrap()
    );
}

#[test]
fn malformed_forms_are_rejected_before_host_parsing() {
    for datatype in [D::Float, D::Double] {
        for text in [
            "", " ", "+", ".", "1e", "1E+", "e1", ".e1", "1e2e3", "1.2.3", "1 e2", "1\t2", "inf",
            "Infinity", "INFINITY", "nan", "+NaN", "-NaN", "NaN(1)", "0x1p0", "1_0", "١", "１",
            "\u{a0}1", "1\u{b}",
        ] {
            assert_eq!(
                datatype.parse_lexical(text),
                Err(E::InvalidLexicalForm),
                "{datatype:?} {text:?}"
            );
        }
    }
}

#[test]
fn binary32_rounding_is_direct_and_ties_go_to_even() {
    for (text, bits) in [
        ("0.1", 0x3dcccccd),
        ("16777217", 0x4b800000),
        ("16777219", 0x4b800002),
        ("1.000000059604644775390625", 0x3f800000),
        ("1.000000178813934326171875", 0x3f800002),
        // Above the midpoint, but rounds to the midpoint in binary64: detect double rounding.
        ("1.0000000596046447753906250000000000000000001", 0x3f800001),
        ("1.1754943508222875e-38", 0x00800000),
        ("1.1754942106924411e-38", 0x007fffff),
        ("1.401298464324817e-45", 1),
        ("3.4028234663852886e38", 0x7f7fffff),
        ("3.4028235677973367e38", 0x7f800000),
    ] {
        assert_eq!(
            D::Float.parse_lexical(text).unwrap(),
            V::from_float(f32::from_bits(bits)),
            "{text}"
        );
    }
}

#[test]
fn binary64_boundaries_and_ties_are_exact() {
    for (text, bits) in [
        ("0.1", 0x3fb999999999999a),
        ("9007199254740993", 0x4340000000000000),
        (
            "1.00000000000000011102230246251565404236316680908203125",
            0x3ff0000000000000,
        ),
        (
            "1.0000000000000001110223024625156540423631668090820312501",
            0x3ff0000000000001,
        ),
        ("2.2250738585072014e-308", 0x0010000000000000),
        ("2.225073858507201e-308", 0x000fffffffffffff),
        ("4.9406564584124654e-324", 1),
        ("1.7976931348623157e308", 0x7fefffffffffffff),
        ("1.7976931348623159e308", 0x7ff0000000000000),
    ] {
        assert_eq!(
            D::Double.parse_lexical(text).unwrap(),
            V::from_double(f64::from_bits(bits)),
            "{text}"
        );
    }
}

#[test]
fn huge_exponents_and_compensating_significands_have_defined_results() {
    let exponent = "9".repeat(2000);
    for datatype in [D::Float, D::Double] {
        let infinity = datatype.parse_lexical("INF").unwrap();
        let zero = datatype.parse_lexical("0").unwrap();
        assert_eq!(
            datatype.parse_lexical(&format!("1e{exponent}")).unwrap(),
            infinity
        );
        assert_eq!(
            datatype.parse_lexical(&format!("1e-{exponent}")).unwrap(),
            zero
        );
        assert_eq!(
            datatype.parse_lexical(&format!("0e{exponent}")).unwrap(),
            zero
        );
        assert_eq!(
            datatype
                .parse_lexical(&format!("1{}e-1000", "0".repeat(1000)))
                .unwrap(),
            datatype.parse_lexical("1").unwrap()
        );
    }
}

#[test]
fn exact_underflow_and_overflow_midpoints_use_ties_to_even() {
    for (datatype, power, zero, subnormal, overflow_shift, mantissa, max, infinity) in [
        (
            D::Float,
            150,
            V::from_float(0.0),
            V::from_float(f32::from_bits(1)),
            103,
            25,
            V::from_float(f32::MAX),
            V::from_float(f32::INFINITY),
        ),
        (
            D::Double,
            1075,
            V::from_double(0.0),
            V::from_double(f64::from_bits(1)),
            970,
            54,
            V::from_double(f64::MAX),
            V::from_double(f64::INFINITY),
        ),
    ] {
        // bounded: independent mathematical fixtures for the two fixed IEEE widths.
        // 5^power * 10^-power is exactly 2^-power, halfway to the least subnormal.
        let coefficient = BigUint::from(5u8).pow(power);
        assert_eq!(
            datatype
                .parse_lexical(&format!("{coefficient}e-{power}"))
                .unwrap(),
            zero
        );
        assert_eq!(
            datatype
                .parse_lexical(&format!("{}e-{power}", &coefficient + 1u8))
                .unwrap(),
            subnormal
        );
        let threshold =
            ((BigUint::from(1u8) << (mantissa as usize)) - 1u8) << (overflow_shift as usize);
        assert_eq!(
            datatype.parse_lexical(&threshold.to_string()).unwrap(),
            infinity
        );
        assert_eq!(
            datatype
                .parse_lexical(&(threshold - 1u8).to_string())
                .unwrap(),
            max
        );
    }
}

#[path = "support/relations.rs"]
mod relations;

#[test]
fn parsed_specials_feed_identity_rules_and_checked_transport() {
    let first = D::Float.parse_lexical("NaN").unwrap();
    let second = D::Float.parse_lexical(" \tNaN\n").unwrap();
    let other_space = D::Double.parse_lexical("NaN").unwrap();
    assert_eq!(first, second);
    assert_ne!(first, other_space);
    let facts = relations::closure(
        &[
            rdf_reasoning::OwlNumericRuleId::DtEq.definition(),
            rdf_reasoning::OwlNumericRuleId::DtDiff.definition(),
        ],
        &[
            relations::Fact::new(
                rdf_reasoning::OWL_NUMERIC_VALUE_RELATION,
                &["first-spelling", "float-nan"],
            ),
            relations::Fact::new(
                rdf_reasoning::OWL_NUMERIC_VALUE_RELATION,
                &["second-spelling", "float-nan"],
            ),
            relations::Fact::new(
                rdf_reasoning::OWL_NUMERIC_VALUE_RELATION,
                &["double-spelling", "double-nan"],
            ),
        ],
    );
    assert!(facts.contains(&relations::Fact::new(
        rdf_reasoning::TRIPLE_RELATION,
        &[
            "first-spelling",
            "http://www.w3.org/2002/07/owl#sameAs",
            "second-spelling"
        ]
    )));
    assert!(facts.contains(&relations::Fact::new(
        rdf_reasoning::TRIPLE_RELATION,
        &[
            "first-spelling",
            "http://www.w3.org/2002/07/owl#differentFrom",
            "double-spelling"
        ]
    )));
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Parsed {
        spelling: String,
        value: V,
    }
    for text in ["NaN", "INF", "-INF", "-0", "1.401298464324817e-45"] {
        let report = Parsed {
            spelling: text.to_owned(),
            value: D::Float.parse_lexical(text).unwrap(),
        };
        assert_eq!(
            serde_json::from_str::<Parsed>(&serde_json::to_string(&report).unwrap()).unwrap(),
            report
        );
        assert_eq!(
            postcard::from_bytes::<Parsed>(&postcard::to_allocvec(&report).unwrap()).unwrap(),
            report
        );
        assert!(report.value.is_member_of(D::Float));
        assert!(
            rdf_reasoning::OwlNumericTypeViolation::check_assertion(text, report.value, D::Integer)
                .is_err()
        );
    }
}
