//! Checked, format-neutral numeric serialization; no bigint wire format leaks.
use super::{ExactNumber, ExactNumberError};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct NumberWire {
    numerator: String,
    denominator: String,
    power_two: i64,
    power_five: i64,
}

impl TryFrom<NumberWire> for ExactNumber {
    type Error = ExactNumberError;
    fn try_from(value: NumberWire) -> Result<Self, Self::Error> {
        Self::normalize(
            Self::integer(&value.numerator)?,
            Self::integer(&value.denominator)?,
            value.power_two,
            value.power_five,
        )
    }
}

impl serde::Serialize for ExactNumber {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        NumberWire {
            numerator: self.numerator.to_string(),
            denominator: self.denominator.to_string(),
            power_two: self.power_two,
            power_five: self.power_five,
        }
        .serialize(serializer)
    }
}
