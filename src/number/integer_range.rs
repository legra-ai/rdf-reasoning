//! Constant-bounded integer reconstruction for the fixed XML Schema ranges.
use super::ExactNumber;
use num_bigint::Sign;

impl ExactNumber {
    pub(crate) fn integer_sign(&self) -> Option<Sign> {
        self.is_integer().then(|| self.numerator.sign())
    }

    pub(crate) fn range_integer(&self) -> Option<i128> {
        if !self.is_integer() || self.power_two > 64 || self.power_five > 28 {
            return None;
        }
        // bounded: fixed-width temporary; every supported finite bound fits in 65 bits.
        // A larger normalized factor cannot be cancelled in an integral value.
        let value = i128::try_from(&self.numerator).ok()?;
        value
            .checked_mul(2i128.checked_pow(self.power_two as u32)?)?
            .checked_mul(5i128.checked_pow(self.power_five as u32)?)
    }
}
