# ADR 0023: Correct whole-term admission in target resource context

This corrects ADR 0021's target-context guidance without changing the shared
rdfs4 clause or introducing a new semantic API. Source occurrences are backed
by source assertions/derivations. A syntactically appearing target triple term,
however, need not have an established denotation: the interpretation function
IT has domain IR x IP x IR. Its predicate must denote a property.

Target adapters therefore admit atomic resources and validated literal values,
with independent source/target blank scopes, but defer a whole triple-term
Resource premise until source evidence or validated construction establishes it.
Continue traversing actual nested components and preserve already established
Resource membership. Do not invent target predicate typing through rdfD2, erase
valid source facts, assert quoted contents or treat deferred work as a negative
entailment verdict. Async authoritative lookup and resource/read errors retain
their existing explicit contracts.

The counterexample source declares every resource a class:
`rdfs:Resource rdfs:subClassOf rdfs:Class`. The target is
`_:x rdfs:subClassOf <<( :a :q :b )>>`. The source need not make q a property,
so the target term need not denote. Incorrectly seeding its Resource membership
would derive Class membership, then reflexive subClassOf, manufacturing a proof.
If that same triple term actually occurs in a source assertion, its established
resource/class membership and reflexive consequence must remain available.

Authored fixture revision 3 adds this negative/positive pair while preserving
all eighteen earlier cases and the existing requirement vocabulary. The engine's
read-only reviewed counterexample is tests/rdfs_context.rs at 6dae93b/da49f02;
the exact JSON documents still require independent consumer parsing/execution.
Catalogue tests validate metadata and package bytes, not full entailment.
No controller implementation or complete RDFS certificate is added here.

Source: https://www.w3.org/TR/2026/CRD-rdf12-semantics-20260924/
section 5, definition of IT and triple-term interpretation; Appendix A remains
explicitly postulated rather than a proof authorizing unsound context seeds.
