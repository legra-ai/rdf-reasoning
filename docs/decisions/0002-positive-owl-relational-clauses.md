# ADR 0002: Share positive OWL relational clauses independently of backend plans

Accepted for the flat positive catalogue.

Extend the checked Datalog catalogue with independently authored positive
clauses for equality, functional and equivalent properties, value restrictions,
equivalent-class membership and schema consequences. Split standard rules with
several conclusions into separately identified single-head clauses. Keep the
full deterministic catalogue discoverable through RuleId::ALL, including entries
not selected by the initial restricted presets.

These are standard relational consequences. Python reference quirks must use
explicit versioned compatibility contracts rather than changing their meaning.
In particular, exact reflexivity and generalized subject/predicate replacement
are semantic requirements; an adapter cannot silently drop them or replace them
with a weaker graph domain. A backend may use indexed kernels when independent
tests establish the same selected consequences and observation boundaries.

The catalogue is not an execution engine and never collects graph facts. It
has no storage dependency or graph I/O. Tests use at most five fixture facts,
check joins independently of the production builders and round-trip every
clause and identifier through JSON and Postcard.

The existing core presets retain their exact selections. Additional catalogue
entries alone do not enable full RDFS, OWL 2 RL, Horst or Python compatibility
names. Complete contracts still require lists, cardinalities, datatypes, axioms,
inconsistency checks and independent conformance evidence.

Provide explicit restricted relational-core presets with all sixty clauses and
with the ten dedicated OWL equality clauses omitted. The latter excludes both
equality generation from property characteristics and equality application;
it does not filter assertions or remove ordinary identity-based Datalog joins.
Expose exact rules and exclusions, preserve the selection through Serde, and
reject full-profile or reference-version names until their semantics exist.

Normative semantic source: OWL 2 Profiles, second edition, section 4.3,
https://www.w3.org/TR/owl2-profiles/#Reasoning_in_OWL_2_RL_and_RDF_Graphs.
