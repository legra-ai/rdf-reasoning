//! Finite OWL numeric value identities, distinct from mathematical compatibility keys.

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
