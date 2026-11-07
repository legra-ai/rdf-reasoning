//! XML Schema floating grammar with direct target-width IEEE conversion.
use crate::{NumericLexicalError, OwlNumericDatatype, OwlNumericValue};

impl OwlNumericDatatype {
    pub(crate) fn parse_floating(
        self,
        lexical: &str,
    ) -> Result<OwlNumericValue, NumericLexicalError> {
        if !Self::floating_grammar(lexical) {
            return Err(NumericLexicalError::InvalidLexicalForm);
        }
        // Convert directly at the target width, avoiding binary64-to-binary32
        // double rounding. Rust's pinned FromStr contract uses roundTiesToEven.
        match self {
            Self::Float => lexical.parse::<f32>().map(OwlNumericValue::from_float),
            Self::Double => lexical.parse::<f64>().map(OwlNumericValue::from_double),
            _ => return Err(NumericLexicalError::UnsupportedDatatype),
        }
        .map_err(|_| NumericLexicalError::InvalidLexicalForm)
    }

    fn floating_grammar(lexical: &str) -> bool {
        if matches!(lexical, "INF" | "+INF" | "-INF" | "NaN") {
            return true;
        }
        let (mantissa, exponent) = match lexical.split_once(['e', 'E']) {
            Some((mantissa, exponent)) => (mantissa, Some(exponent)),
            None => (lexical, None),
        };
        if exponent.is_some_and(|text| !Self::floating_digits(Self::unsigned_lexical(text))) {
            return false;
        }
        let mantissa = Self::unsigned_lexical(mantissa);
        match mantissa.split_once('.') {
            Some((whole, fraction)) => {
                (!whole.is_empty() || !fraction.is_empty())
                    && whole.bytes().all(|byte| byte.is_ascii_digit())
                    && fraction.bytes().all(|byte| byte.is_ascii_digit())
            }
            None => Self::floating_digits(mantissa),
        }
    }

    fn unsigned_lexical(text: &str) -> &str {
        text.strip_prefix(['+', '-']).unwrap_or(text)
    }

    fn floating_digits(text: &str) -> bool {
        !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit())
    }
}
