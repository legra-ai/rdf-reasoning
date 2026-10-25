# ADR 0006: Separate finite OWL numeric identity from mathematical equality

Accepted for a finite value primitive, not a complete datatype evaluator.

ExactNumber intentionally normalizes mathematical equality across representations
and collapses signed zero. That remains the right contract for caller-selected
Python-reference keys. It is insufficient for standard OWL numeric identities.
OWL 2 syntax section 4.2 distinguishes decimal, float and double value spaces and
floating-point signed zeros. Do not change ExactNumber to impose that policy on
its existing consumers.

Provide OwlFiniteNumericValue with explicit decimal, binary32 float and binary64
double construction. Decimal construction delegates exact normalization and
exponent checks. Floating identities retain exact bits, including signed zero
and subnormals. Value-space tags prevent cross-space identity. Equality and Hash
compare this value identity; neither implements facet ordering or IEEE arithmetic
comparison. No source RDF term is replaced.

The supported domain is explicitly finite. Constructors and decoders reject
infinities and NaN; they do not map them to finite values. Checked Serde rejects
non-finite bit patterns, non-decimal rational payloads, unknown variants and
unknown fields. Both JSON and Postcard round trips preserve identity. Decimal
wire values may normalize equivalent forms, but must remain finite decimals.

This adds no lexical parser, datatype-membership operation, Boolean coercion,
graph storage, rules, presets or profile claims. dt-type2, dt-eq, dt-diff and
dt-not-type still need complete selected value-domain policies and adapter
execution. In particular, Python compatibility modes must not silently switch
to this standard identity primitive. Forward and backward adapters can consume
it independently, without depending on either engine's private storage.

Reference: https://www.w3.org/TR/2012/REC-owl2-syntax-20121211/#Floating-Point_Numbers
