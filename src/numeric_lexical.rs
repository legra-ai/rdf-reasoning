//! Shared XML Schema numeric lexical mapping, independent of RDF storage.
use crate::{ExactNumber, OwlNumericDatatype, OwlNumericValue};
use num_bigint::Sign;
use std::fmt;

/// Failure to interpret a literal under a selected numeric datatype.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NumericLexicalError {
    /// The spelling violates the selected XML Schema lexical grammar.
    InvalidLexicalForm,
    /// A grammatically valid integer is outside its declared datatype's range.
    OutOfRange,
    /// Lexical interpretation is not supported for the selected datatype.
    UnsupportedDatatype,
    /// The normalized value exceeds the compact signed-exponent representation.
    RepresentationOverflow,
}

impl fmt::Display for NumericLexicalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLexicalForm => "invalid XML Schema numeric lexical form",
            Self::OutOfRange => "integer value outside declared datatype range",
            Self::UnsupportedDatatype => "lexical interpretation unsupported for selected datatype",
            Self::RepresentationOverflow => "numeric exponent exceeds supported representation",
        })
    }
}
impl std::error::Error for NumericLexicalError {}

impl OwlNumericDatatype {
    /// Interpret an XML Schema numeric lexical form at its declared precision.
    ///
    /// Applies XML whitespace collapse; internal whitespace and Unicode digits
    /// cannot match these grammars. Integer/decimal parsing is exact and forbids
    /// exponent notation. Float/double accept decimal/scientific notation plus
    /// INF, +INF, -INF and NaN. They round directly to binary32/binary64 using
    /// nearest, ties-to-even, including signed zeros, subnormals and infinities.
    /// Real/rational lexical interpretation remains unsupported.
    /// Source literal identity/spelling remains caller-owned.
    ///
    /// Allocation scales with one lexical value, never a graph; callers own
    /// input size/decoder budgets. Fixed-range integers are never truncated.
    ///
    /// # Errors
    /// Rejects unsupported selectors, invalid grammar, derived range violations
    /// and compact representation overflow as distinct typed errors.
    pub fn parse_lexical(self, lexical: &str) -> Result<OwlNumericValue, NumericLexicalError> {
        let lexical = lexical.trim_matches([' ', '\t', '\r', '\n']);
        if matches!(self, Self::Float | Self::Double) {
            return self.parse_floating(lexical);
        }
        if self != Self::Decimal && !self.is_integer_datatype() {
            return Err(NumericLexicalError::UnsupportedDatatype);
        }
        let unsigned = lexical.strip_prefix(['+', '-']).unwrap_or(lexical);
        let mut digits = 0usize;
        let mut point = None;
        for (index, byte) in unsigned.bytes().enumerate() {
            if byte.is_ascii_digit() {
                digits += 1;
            } else if byte == b'.' && self == Self::Decimal && point.is_none() {
                point = Some(index);
            } else {
                return Err(NumericLexicalError::InvalidLexicalForm);
            }
        }
        if digits == 0 {
            return Err(NumericLexicalError::InvalidLexicalForm);
        }
        let scale = point.map_or(0, |index| unsigned.len() - index - 1);
        let exponent =
            i64::try_from(scale).map_err(|_| NumericLexicalError::RepresentationOverflow)?;
        // bounded: one caller-supplied lexical value; no graph data is retained.
        let coefficient = lexical.replace('.', "");
        let value = OwlNumericValue::try_from_decimal(&coefficient, -exponent)
            .map_err(|_| NumericLexicalError::RepresentationOverflow)?;
        if !value.is_member_of(self) {
            return Err(NumericLexicalError::OutOfRange);
        }
        Ok(value)
    }

    pub(crate) fn is_integer_datatype(self) -> bool {
        !matches!(
            self,
            Self::Decimal | Self::Rational | Self::Real | Self::Float | Self::Double
        )
    }

    pub(crate) fn contains_integer(self, value: &ExactNumber) -> bool {
        let Some(sign) = value.integer_sign() else {
            return false;
        };
        match self {
            Self::Integer => true,
            Self::NonPositiveInteger => sign != Sign::Plus,
            Self::NegativeInteger => sign == Sign::Minus,
            Self::NonNegativeInteger => sign != Sign::Minus,
            Self::PositiveInteger => sign == Sign::Plus,
            _ => self.integer_bounds().is_some_and(|(min, max)| {
                value
                    .range_integer()
                    .is_some_and(|integer| (min..=max).contains(&integer))
            }),
        }
    }

    fn integer_bounds(self) -> Option<(i128, i128)> {
        Some(match self {
            Self::Long => (i128::from(i64::MIN), i128::from(i64::MAX)),
            Self::Int => (i128::from(i32::MIN), i128::from(i32::MAX)),
            Self::Short => (i128::from(i16::MIN), i128::from(i16::MAX)),
            Self::Byte => (i128::from(i8::MIN), i128::from(i8::MAX)),
            Self::UnsignedLong => (0, i128::from(u64::MAX)),
            Self::UnsignedInt => (0, i128::from(u32::MAX)),
            Self::UnsignedShort => (0, i128::from(u16::MAX)),
            Self::UnsignedByte => (0, i128::from(u8::MAX)),
            _ => return None,
        })
    }
}
