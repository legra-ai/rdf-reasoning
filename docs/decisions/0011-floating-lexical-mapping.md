# ADR 0011: Checked floating lexical grammar and direct IEEE conversion

Accepted for XML Schema float and double interpretation.

Extend the shared parse_lexical entry point with the XML Schema decimal/scientific
floating grammar and exact INF, +INF, -INF and NaN spellings. Apply outer XML
whitespace trimming through the existing entry point. Validate ASCII grammar
before host parsing: Rust also accepts case-insensitive infinity and signed NaN
spellings that are outside this contract. No regex dependency or fallback is
needed; numeric mantissa and exponent validation scans the caller's input.

After validation, use the pinned Rust FromStr implementation directly at each
target precision. Its documented nearest, ties-to-even semantics determine the
result. Never parse binary32 through binary64 or decimal formatting: that would
introduce double rounding. Overflow produces signed infinity; underflow follows
the same rounding into subnormal or signed zero. Exact zero retains lexical
sign even with enormous exponents. No explicit power expansion is performed.
Input/decoder budgets remain caller-owned; allocation cannot scale with a graph.

Construct OwlNumericValue through its existing floating constructors so signed
zeros and float/double spaces remain distinct and NaN has one identity per space.
Existing checked transport and membership/diagnostic contracts consume these
values. Real/rational lexical interpretation stays explicitly unsupported.
No preset, new dependency, production executor or consumer adapter is added.

Tests precede implementation. Independent hexadecimal bit expectations guard
rounding, finite limits and normal/subnormal transitions. Exact rational midpoint
fixtures derive 2^-150 and 2^-1075 mathematically and distinguish halfway from
just-above-halfway results, plus exact overflow thresholds. A decimal just above
a binary32 midpoint catches binary64 intermediate rounding. Malformed lexical
forms, XML whitespace and huge exponents are covered separately.

References:
- XML Schema 1.1 Datatypes sections 3.3.4 and 3.3.5:
  https://www.w3.org/TR/xmlschema11-2/#float
- Rust FromStr f32 target-width rounding contract:
  https://doc.rust-lang.org/std/primitive.f32.html#impl-FromStr-for-f32
