# ADR 0010: Exact integer/decimal lexical mapping and derived ranges

Accepted for XML Schema decimal, integer and its twelve built-in derived types.

Centralize lexical interpretation on OwlNumericDatatype. Trim only XML space,
tab, carriage return and newline; internal whitespace remains invalid after
collapse for these grammars. Require optional ASCII sign and at least one ASCII
digit. Decimal admits one decimal point, integer datatypes do not. Never use
host floating parsing, Unicode digit/whitespace predicates, exponent notation,
coercion or a fallback for malformed literals. Keep source lexical identity
caller-owned while constructing exact normalized decimal-space values.

Add all twelve XML Schema integer-derived selectors to the same membership
contract, including their exact IRIs and checked Serde identities. Fixed bounds
use checked fixed-width temporaries only after normalized integrality checks.
Factors exceeding the largest finite bound reject immediately; never expand a
huge exponent. Unbounded sign restrictions use the normalized numerator sign.
No graph-sized collection or digit cap is introduced; allocation belongs to one
literal and callers own input-size budgets. Compact signed exponents retain the
existing explicit representation limit rather than silently approximating.

A well-formed integer lexical form outside its declared range produces a typed
range error. Invalid syntax, unsupported datatype interpretation and numeric
representation overflow are distinct errors with format-neutral Serde. Float,
double, rational and real lexical mapping remains explicitly unsupported.

Existing membership clauses and typed mismatch diagnostics use the extended
shared membership operation directly. This does not introduce a parser fallback,
consumer adapter or complete OWL-RL preset. The datatype enum is expanded in
place; exhaustive downstream matches must handle the new selectors.

Tests first cover equivalent spellings, invalid ASCII/Unicode/whitespace forms,
all finite endpoints and adjacent invalid values, negative zero, unbounded
integers, extreme compact exponents and JSON/Postcard transport.

Normative source: XML Schema 1.1 Datatypes sections 3.3.3 and 3.4.13–3.4.25:
https://www.w3.org/TR/xmlschema11-2/
