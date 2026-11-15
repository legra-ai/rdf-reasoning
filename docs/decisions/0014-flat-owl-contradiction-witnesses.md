# ADR 0014: Flat OWL contradiction clauses as explicit diagnostic relations

Implement seven missing flat OWL 2 RL/RDF Second Edition (2012) contradiction
clauses through existing checked positive Datalog Rule definitions. The Datalog
owner agreed to dedicated non-RDF auxiliary witness relations, following the
published numeric type-violation pattern. A generic denial-constraint IR is a
separate contract, not a prerequisite for these semantic catalogue entries.

Use stable rule identifiers and distinct urn:rdf-reasoning:owl-violation:
relation names. Their ordered projections are:

| Clause | Witness operands |
| --- | --- |
| eq-diff1 | x, y |
| prp-irp | p, x |
| prp-asyp | p, x, y |
| prp-pdw | p1, p2, x, y |
| cax-dw | c1, c2, x |
| cls-com | c1, c2, x |
| cls-nothing2 | x |

Preserve exact bound RDF identity and selected source scope. Every premise is
positive evidence. Missing facts never prove inconsistency; do not use closed
world negation, a unique-name assumption, host numeric comparison or extra
inequality guards. eq-diff1 matches asserted or derived sameAs and differentFrom
for the same ordered pair. Equality generation/replacement is separately
selected; a consumer checking consistency must include its selected equality
closure. No representative-compression or cross-scope join is implied.

Heads are diagnostic relations, not RDF triples containing false, owl:Nothing
assertions or ordinary inferred statements. Consumers must explicitly admit the
diagnostic contract and implement bounded reporting/termination. Ordinary
positive Datalog syntax validation does not certify witness execution or an
inconsistency reporter. Keep all existing preset selections and exclusions.

Identical tuples have ordinary relation-set identity. Reporting order, witness
multiplicity, explanations and provenance are not encoded; consumers own these
policies and must not materialize a graph-sized witness report. Only finite
metadata is allocated here: seven rules with at most three body atoms and four
head operands. No evaluator or graph storage enters this crate.

Test-first independent fixtures cover every clause, removal of every essential
premise, incorrect joins and empty input. Checked nested JSON/Postcard tests
reject unsafe heads and unknown identities. These seven entries do not establish
complete OWL-RL support: list-dependent violations, cardinalities, negative
property assertions, remaining datatype semantics and executable consumers are
still separate obligations.

Source: https://www.w3.org/TR/2012/REC-owl2-profiles-20121211/#Reasoning_in_OWL_2_RL_and_RDF_Graphs
