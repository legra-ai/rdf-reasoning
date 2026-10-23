# ADR 0005: Share exact finite numeric keys independently of lexical interpretation

Accepted for a pure value primitive supporting the compatibility effort.

RDFS Python-reference substitution compares cached values rather than RDF term
identity. Native string substitution does not cover equal integer/decimal/float
values. Those semantics cannot be implemented through rounded decimal formatting
or a fixed-width integer representation without changing consequences.

Provide ExactNumber as a canonical reduced rational coefficient and separate
signed powers of two and five. Remove those factors from both numerator and
denominator, normalize sign and zero, and check exponent arithmetic. Binary64
construction uses exact bits. Large decimal powers stay compact; never expand
10^exponent merely to build an equality key. Arbitrary precision belongs to one
caller-owned numeric value, never a graph-sized collection.

Keep lexical parsing and datatype-specific validity separate. The same finite
value primitive can serve standard or explicitly versioned Python semantics;
it does not equate boolean and numeric datatype values by itself. Non-finite
values and Python conversion/context behavior are not silently approximated.

Serde uses checked string integer operands and signed exponent fields in both
text and binary formats. Decoding calls the same normalization as constructors.
Source RDF literal lexical identity remains unchanged. Existing preset rules,
exclusions and complete-profile rejection remain unchanged.

LiteralRuleId separately supplies the positive object-substitution clause over
an explicit binary value-class relation. Adapters own class interpretation and
must reject unsupported relations instead of ignoring them. A key relation is
not a claim that arbitrary Python equality is transitive or reflexive, nor a
substitute for context-dependent errors and special values. Keep such behavior
explicitly versioned and outside the generic numeric key.

In particular, OWL 2 syntax §4.2 keeps float, double and decimal value spaces
disjoint. Standard consumers must tag mathematical keys by that value space;
Python-reference RDFS may intentionally compare their cached numeric values.
Do not expose one of those policies under the other's complete-profile name.
