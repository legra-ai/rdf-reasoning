//! Normalize one caller-owned rational value without expanding large powers.
use super::{ExactNumber, ExactNumberError};
use num_bigint::{BigInt, BigUint, Sign};

impl ExactNumber {
    pub(super) fn normalize(
        numerator: BigInt,
        denominator: BigInt,
        mut power_two: i64,
        mut power_five: i64,
    ) -> Result<Self, ExactNumberError> {
        let (numerator_sign, mut numerator) = numerator.into_parts();
        let (denominator_sign, mut denominator) = denominator.into_parts();
        if denominator == BigUint::ZERO {
            return Err(ExactNumberError::ZeroDenominator);
        }
        if numerator == BigUint::ZERO {
            return Ok(Self {
                numerator: BigInt::from(0),
                denominator: BigUint::from(1u8),
                power_two: 0,
                power_five: 0,
            });
        }
        // bounded: both operands and the Euclidean temporaries belong to one numeric value.
        let common = Self::gcd(numerator.clone(), denominator.clone());
        numerator /= &common;
        denominator /= common;
        let numerator_two = numerator.trailing_zeros().expect("nonzero numerator");
        let denominator_two = denominator.trailing_zeros().expect("nonzero denominator");
        numerator >>= numerator_two;
        denominator >>= denominator_two;
        let (numerator, numerator_five) = Self::remove_fives(numerator)?;
        let (denominator, denominator_five) = Self::remove_fives(denominator)?;
        power_two = Self::shift(power_two, numerator_two, denominator_two)?;
        power_five = Self::shift(power_five, numerator_five, denominator_five)?;
        let sign = if numerator_sign == denominator_sign {
            Sign::Plus
        } else {
            Sign::Minus
        };
        Ok(Self {
            numerator: BigInt::from_biguint(sign, numerator),
            denominator,
            power_two,
            power_five,
        })
    }

    fn gcd(mut left: BigUint, mut right: BigUint) -> BigUint {
        while right != BigUint::ZERO {
            let remainder = left % &right;
            left = right;
            right = remainder;
        }
        left
    }

    fn remove_fives(mut value: BigUint) -> Result<(BigUint, u64), ExactNumberError> {
        let mut power = 0u64;
        // bounded: two fixed factors; 5^27 fits in u64 and removes long zero tails in chunks.
        for (factor, count) in [(7_450_580_596_923_828_125u64, 27u64), (5, 1)] {
            while &value % factor == BigUint::ZERO {
                value /= factor;
                power = power
                    .checked_add(count)
                    .ok_or(ExactNumberError::ExponentOverflow)?;
            }
        }
        Ok((value, power))
    }

    fn shift(power: i64, numerator: u64, denominator: u64) -> Result<i64, ExactNumberError> {
        let difference = i128::from(numerator) - i128::from(denominator);
        i64::try_from(i128::from(power) + difference)
            .map_err(|_| ExactNumberError::ExponentOverflow)
    }
}
