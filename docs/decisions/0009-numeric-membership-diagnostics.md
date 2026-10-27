# ADR 0009: Numeric membership and explicit incompatible-type witnesses

Accepted for the existing interpreted decimal/float/double domain.

Share numeric/dt-type2 and numeric/dt-not-type as checked positive clauses,
separate from numeric identity rules and all existing presets. Bind explicit
binary membership and nonmembership evidence with exact source literal and
datatype IRI identities. The adapter supplies evidence from the shared numeric
membership operation after lexical validation, in one selected graph scope.
Absence from a partial relation is not a negative fact. Unknown or unsupported
interpretation must fail explicitly rather than become nonmembership.

Membership emits generalized rdf:type assertions. Nonmembership joined to a
matching asserted rdf:type emits a separate binary diagnostic witness relation.
This is the dt-not-type contradiction represented as a witness: do not insert
an RDF triple with a false/null predicate or substitute an owl:Nothing type.
The executor must treat a witness as inconsistency under the selected contract;
reporting and termination remain caller-owned. Missing evidence, an assertion
for another literal, or nonmembership in another datatype cannot trigger it.

Expose the six exact datatype IRIs through OwlNumericDatatype. Decimal, integer,
float and double are supported OWL-RL datatypes; real and rational are general
OWL extensions explicitly excluded by OWL-RL. A caller selecting OWL-RL must
exclude those two, even though general numeric membership supports them. This
slice does not implement derived integer ranges or other datatype domains.

OwlNumericTypeViolation<L> owns a caller-defined exact literal identity, an
interpreted value and an asserted datatype. check_assertion returns the typed
violation only on nonmembership. Checked JSON/Postcard decoding applies the
same invariant and rejects unknown fields or invalid nested numeric values.
The report validates a value-space mismatch, not a lexical mapping or the
presence of an assertion in an external graph; these are adapter obligations.

Allocate only fixed rule metadata or one diagnostic value. No graph collection,
backend, filesystem I/O or global state enters the library. Forward/backward
executors bind indexed relations or equivalent streaming kernels. Preserve
generalized type assertions through inference, then perform RDF projection.
Independent tiny fixtures and malformed report tests precede implementation.

Normative source: OWL 2 Profiles, sections 4.2.1 and 4.3 Table 8:
https://www.w3.org/TR/2012/REC-owl2-profiles-20121211/
Numeric membership and identity remain defined by ADR 0007; no full-profile
claim or consumer integration is introduced.
