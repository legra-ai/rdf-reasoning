//! OWL numeric value identities, distinct from mathematical compatibility keys.

use crate::{ExactNumber, ExactNumberError};

/// A finite numeric value identity in the OWL decimal, float or double space.
///
/// The three spaces are disjoint. Floating-point signed zeros retain distinct
/// identities. Decimal-derived integer values share the decimal space. This
/// type does not parse datatype lexical forms, implement datatype membership,
/// or include infinities/NaN. Existing presets are unaffected.
#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Deserialize)]
#[serde(try_from = "NumericWire")]
pub struct OwlFiniteNumericValue(NumericWire);

#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
enum NumericWire {
    Decimal(ExactNumber),
    Float(u32),
    Double(u64),
}

impl OwlFiniteNumericValue {
    /// Construct a decimal-space value coefficient × 10^exponent.
    ///
    /// # Errors
    /// Rejects invalid ASCII integer coefficients and normalized exponent overflow.
    pub fn try_from_decimal(coefficient: &str, exponent: i64) -> Result<Self, ExactNumberError> {
        ExactNumber::try_from_decimal(coefficient, exponent)
            .map(|value| Self(NumericWire::Decimal(value)))
    }

    /// Construct an OWL float-space identity from an already interpreted binary32 value.
    ///
    /// # Errors
    /// Rejects infinities and NaN, outside this explicitly finite domain.
    pub fn try_from_float(value: f32) -> Result<Self, ExactNumberError> {
        if !value.is_finite() {
            return Err(ExactNumberError::NonFinite);
        }
        Ok(Self(NumericWire::Float(value.to_bits())))
    }

    /// Construct an OWL double-space identity from an already interpreted binary64 value.
    ///
    /// # Errors
    /// Rejects infinities and NaN, outside this explicitly finite domain.
    pub fn try_from_double(value: f64) -> Result<Self, ExactNumberError> {
        if !value.is_finite() {
            return Err(ExactNumberError::NonFinite);
        }
        Ok(Self(NumericWire::Double(value.to_bits())))
    }
}

impl TryFrom<NumericWire> for OwlFiniteNumericValue {
    type Error = &'static str;

    fn try_from(value: NumericWire) -> Result<Self, Self::Error> {
        let valid = match &value {
            NumericWire::Decimal(value) => value.is_finite_decimal(),
            NumericWire::Float(bits) => f32::from_bits(*bits).is_finite(),
            NumericWire::Double(bits) => f64::from_bits(*bits).is_finite(),
        };
        if !valid {
            return Err("expected a finite value in the selected OWL numeric space");
        }
        Ok(Self(value))
    }
}

impl serde::Serialize for OwlFiniteNumericValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

/// A numeric datatype supported by [`OwlNumericValue::is_member_of`].
///
/// This is a restricted selector, not the complete OWL datatype map. Rational
/// and real membership can be queried, but this value model constructs only
/// decimal, float and double values; non-decimal rationals are not represented.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OwlNumericDatatype {
    /// XML Schema decimal.
    Decimal,
    /// XML Schema integer, a subset of decimal.
    Integer,
    /// OWL rational, containing decimal.
    Rational,
    /// OWL real, containing rational but disjoint from float and double.
    Real,
    /// XML Schema binary32 float, including signed zeros, infinities and NaN.
    Float,
    /// XML Schema binary64 double, including signed zeros, infinities and NaN.
    Double,
}

/// A decimal, float or double OWL value identity, including non-finite floats.
///
/// Equality and hashing mean datatype value identity, not IEEE arithmetic
/// equality or facet comparison. Each floating space has one NaN identity:
/// payload, sign and signaling bits do not denote separate XML Schema values.
/// Signed zeros and the three value spaces remain distinct. This does not parse
/// literals, implement facets, or enable datatype rules or complete presets.
#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Deserialize)]
#[serde(try_from = "NumericWire")]
pub struct OwlNumericValue(NumericWire);

impl OwlNumericValue {
    // XML Schema has a single NaN value in each floating space. Use fixed
    // canonical quiet-NaN encodings, independent of the host's NAN constants.
    const FLOAT_NAN: u32 = 0x7fc00000;
    const DOUBLE_NAN: u64 = 0x7ff8000000000000;

    /// Construct a decimal-space value coefficient × 10^exponent.
    ///
    /// # Errors
    /// Rejects invalid ASCII integer coefficients and normalized exponent overflow.
    pub fn try_from_decimal(coefficient: &str, exponent: i64) -> Result<Self, ExactNumberError> {
        ExactNumber::try_from_decimal(coefficient, exponent)
            .map(|value| Self(NumericWire::Decimal(value)))
    }

    /// Construct a float-space identity, canonicalizing all NaN encodings.
    pub fn from_float(value: f32) -> Self {
        Self(NumericWire::Float(if value.is_nan() {
            Self::FLOAT_NAN
        } else {
            value.to_bits()
        }))
    }

    /// Construct a double-space identity, canonicalizing all NaN encodings.
    pub fn from_double(value: f64) -> Self {
        Self(NumericWire::Double(if value.is_nan() {
            Self::DOUBLE_NAN
        } else {
            value.to_bits()
        }))
    }

    /// Test membership in a selected datatype's value space, without coercion.
    ///
    /// Decimal values belong to rational and real, and to integer exactly when
    /// integral. Float and double values belong only to their own selected
    /// space, even for equal magnitudes, zeros, infinities or NaN. This query
    /// evaluates interpreted values, not literal lexical validity.
    pub fn is_member_of(&self, datatype: OwlNumericDatatype) -> bool {
        match (&self.0, datatype) {
            (NumericWire::Decimal(value), OwlNumericDatatype::Integer) => value.is_integer(),
            (
                NumericWire::Decimal(_),
                OwlNumericDatatype::Decimal
                | OwlNumericDatatype::Rational
                | OwlNumericDatatype::Real,
            )
            | (NumericWire::Float(_), OwlNumericDatatype::Float)
            | (NumericWire::Double(_), OwlNumericDatatype::Double) => true,
            _ => false,
        }
    }
}

impl TryFrom<NumericWire> for OwlNumericValue {
    type Error = &'static str;

    fn try_from(value: NumericWire) -> Result<Self, Self::Error> {
        let valid = match &value {
            NumericWire::Decimal(value) => value.is_finite_decimal(),
            NumericWire::Float(bits) => !f32::from_bits(*bits).is_nan() || *bits == Self::FLOAT_NAN,
            NumericWire::Double(bits) => {
                !f64::from_bits(*bits).is_nan() || *bits == Self::DOUBLE_NAN
            }
        };
        if !valid {
            return Err("expected decimal or canonical floating value identity");
        }
        Ok(Self(value))
    }
}

impl serde::Serialize for OwlNumericValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}
