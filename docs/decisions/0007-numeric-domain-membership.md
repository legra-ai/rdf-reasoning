# ADR 0007: Explicit non-finite identity and restricted numeric membership

Accepted for the decimal/float/double interpreted-value domain.

Provide OwlNumericValue separately from the explicitly finite type. Reuse its
private tagged wire and exact decimal normalization. Preserve signed zeros,
finite bit identity, infinity signs and disjoint decimal/float/double spaces.
Each floating space contains one NaN value: machine payload, sign and signaling
variations canonicalize at construction. Identity is reflexive, including NaN;
this is not IEEE arithmetic equality, ordering or facet evaluation.

Use fixed canonical NaN bits in serialization. Checked JSON and Postcard decoding
reject other NaN bit encodings, non-decimal rational payloads, unknown variants
and unknown fields. Do not silently accept an alternate serialized identity.
Neither transport is claimed to be an RDF canonical encoding.

Expose membership through a restricted OwlNumericDatatype selector for decimal,
integer, rational, real, float and double. Every represented decimal belongs to
rational and real; integer membership checks normalized denominator and powers
without expanding exponents. Float and double are disjoint from each other and
real even for zeros, integral magnitudes, infinities and NaN. Unknown datatype
selectors fail deserialization rather than returning a guessed membership.

The value domain constructs decimal/float/double only, not all rationals or
irrational reals. Derived integer bounds, other datatypes, lexical validity,
facets, dt-type2, dt-eq, dt-diff and dt-not-type execution remain unsupported.
No preset or existing compatibility key changes. No graph storage, I/O,
backend dependency or graph-sized collection is introduced.

Independent fixtures cover signaling/quiet NaN payloads, cross-space identity,
signed zeros, infinities, membership, compact extreme decimal powers, malformed
wires and nested JSON/Postcard round trips.

Sources:
- OWL 2 Syntax sections 4.1–4.2:
  https://www.w3.org/TR/2012/REC-owl2-syntax-20121211/#Datatype_Maps
- XML Schema 1.1 Datatypes sections 3.3.4–3.3.5 (one NaN value,
  identity distinct from arithmetic equality):
  https://www.w3.org/TR/xmlschema11-2/#float
