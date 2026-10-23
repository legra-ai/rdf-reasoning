//! Exact finite numeric values, independent of RDF term identity and lexical parsing.
use num_bigint::{BigInt, BigUint};
use std::fmt;

mod normalization;
mod representation;
use representation::NumberWire;

/// A canonical exact finite number used for value-equality keys.
///
/// The value is numerator/denominator × 2^power_two × 5^power_five. Numerator
/// and denominator are coprime and have no factors of two or five. Exponents
/// stay compact even for enormous decimal scales. This never defines RDF term
/// identity, datatype membership or Python-specific literal conversion rules.
#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Deserialize)]
#[serde(try_from = "NumberWire")]
pub struct ExactNumber {
    numerator: BigInt,
    denominator: BigUint,
    power_two: i64,
    power_five: i64,
}

/// An invalid exact-number construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExactNumberError {
    /// Integer operands must contain optional ASCII sign and ASCII digits.
    InvalidInteger,
    /// A fraction cannot have a zero denominator.
    ZeroDenominator,
    /// A normalized power of two or five exceeds the supported signed exponent.
    ExponentOverflow,
    /// NaN and infinities are outside this finite numeric value domain.
    NonFinite,
}

impl fmt::Display for ExactNumberError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidInteger => "expected ASCII signed integer digits",
            Self::ZeroDenominator => "exact number denominator must be nonzero",
            Self::ExponentOverflow => "normalized numeric exponent exceeds i64",
            Self::NonFinite => "NaN and infinity are not exact finite numbers",
        })
    }
}
impl std::error::Error for ExactNumberError {}

impl ExactNumber {
    /// Construct an exact integer, preserving its mathematical value.
    ///
    /// # Errors
    /// Returns an error for invalid signed ASCII integer digits or exponent overflow.
    pub fn try_from_integer(integer: &str) -> Result<Self, ExactNumberError> {
        Self::try_from_fraction(integer, "1")
    }

    /// Construct an exact fraction; signs and common factors are normalized.
    ///
    /// # Errors
    /// Returns an error for invalid integer operands, zero denominator or exponent overflow.
    pub fn try_from_fraction(numerator: &str, denominator: &str) -> Result<Self, ExactNumberError> {
        Self::normalize(Self::integer(numerator)?, Self::integer(denominator)?, 0, 0)
    }

    /// Construct coefficient × 10^exponent without expanding the power of ten.
    ///
    /// Lexical datatype parsing and any caller-specific digit limits happen
    /// before this value construction. The coefficient uses signed ASCII digits.
    ///
    /// # Errors
    /// Returns an error for an invalid coefficient or normalized exponent overflow.
    pub fn try_from_decimal(coefficient: &str, exponent: i64) -> Result<Self, ExactNumberError> {
        Self::normalize(
            Self::integer(coefficient)?,
            BigInt::from(1),
            exponent,
            exponent,
        )
    }

    /// Preserve the exact finite binary64 value, including subnormal numbers.
    ///
    /// This does not parse an RDF lexical form or impose binary32 semantics.
    ///
    /// # Errors
    /// Rejects NaN and infinities.
    pub fn try_from_f64(value: f64) -> Result<Self, ExactNumberError> {
        let bits = value.to_bits();
        let exponent = ((bits >> 52) & 0x7ff) as i64;
        if exponent == 0x7ff {
            return Err(ExactNumberError::NonFinite);
        }
        let mut significand = bits & ((1u64 << 52) - 1);
        let exponent = if exponent == 0 {
            -1074
        } else {
            significand |= 1u64 << 52;
            exponent - 1023 - 52
        };
        let mut numerator = BigInt::from(significand);
        if bits >> 63 != 0 {
            numerator = -numerator;
        }
        Self::normalize(numerator, BigInt::from(1), exponent, 0)
    }

    fn integer(value: &str) -> Result<BigInt, ExactNumberError> {
        let digits = value.strip_prefix(['+', '-']).unwrap_or(value);
        if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(ExactNumberError::InvalidInteger);
        }
        BigInt::parse_bytes(value.as_bytes(), 10).ok_or(ExactNumberError::InvalidInteger)
    }
}

impl fmt::Display for ExactNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}/{}*2^{}*5^{}",
            self.numerator, self.denominator, self.power_two, self.power_five
        )
    }
}
