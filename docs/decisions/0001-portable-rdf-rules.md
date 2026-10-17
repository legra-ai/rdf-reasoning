# ADR 0001: Portable RDF rules, independent of execution

Accepted for the initial implementation.

The generic Datalog structural model describes a rule language. RDF vocabulary
semantics and named rule selections belong in this crate. Backend identity,
storage, scheduling, graph scope and execution remain adapter responsibilities.
An OWL ontology tree is not required to execute rules directly over RDF tuples.

Start with concrete restricted presets whose complete clauses can be inspected
and tested. Each selector identifies an exact finite rule list and explicit
exclusions. Never claim full RDFS, OWL 2 RL, Horst, vendor or Python compatibility
from a name alone. Complete profiles require axioms, datatype semantics and
inconsistency coverage as appropriate, plus independent conformance evidence.
Equality generation, reflexivity and substitution are semantics; representative
compression is an execution choice only if observable consequences agree.

Use one generalized three-place assertion relation so predicate variables and
schema assertions share the same closure. Standard RDF projection occurs after
reasoning. Adapter term identities must be injective and preserve literal
lexical forms and blank-node scopes. Extension terms are opaque unless a rule
explicitly defines their semantics. No silent unsupported-feature fallback.

The public catalogue uses checked Datalog constructors, finite metadata and
format-neutral Serde. No engine backend, graph hydration, synchronous I/O or
process-global mutable state enters the crate. Finite fixtures exercise rule
consequences independently of the builders. Generic Datalog capability checks
complement, and do not replace, RDF adapter validation.
