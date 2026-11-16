# ADR 0021: Trusted recursive RDF 1.2 occurrence relations

The engine and Datalog owners agreed two semantic auxiliary relations and checked
definitions, independent of existing restricted presets. Rdf12OccurrenceRuleId
enumerates RdfD2 and Rdfs4 with exact serialized clause names rdfD2 and rdfs4.
ALL selects both source rules. Each definition has one positive virtual premise:
appearing-triple(subject,predicate,object) types the predicate rdf:Property;
appearing-term(term) types the term rdfs:Resource. No model or executor is added.

An appearing source triple may be an assertion, a source-derived tuple or a
triple term recursively appearing in those roots. Appearing terms include the
subject, predicate and object, including the triple term itself and its nested
components. Generalized intermediate positions must retain identity. Literal
metadata is not an additional appearing term; an internal datatype IRI needs
its own syntactic occurrence to enter the appearing-term relation.

These are trusted semantic premises, not user RDF. Never expose quoted contents
as asserted triple tuples or fabricate asserted evidence for them. Preserve root
identity, recursive path, selected role, snapshot and origin separately. User RDF
whose predicate is an auxiliary relation IRI cannot forge relation membership.

The source snapshot includes selected asserted and derived facts. Appendix A
step 4 selects only rdfs4 from this pair for target context, not rdfD2. Target
context therefore never creates property instances merely because a predicate
occurs in the target, nor does it promote target assertions into the source.
All appearing target terms, including blanks, can supply contextual resource
typing. Standardize source and target blank identities apart, including nested
positions; repeated target blanks retain one key. They remain existential target
variables in final matching, not Skolem/fixed IRIs or arbitrary source properties.
Keep target context out of source consistency and assertion provenance.

Adapters must traverse authoritative immutable snapshots asynchronously with
bounded path, work and nesting budgets. Asserted and inferred facts can be
evicted and loaded on demand. A cache miss is not an exhausted relation. Surface
read/cancellation/resource/unsupported-term errors explicitly instead of silently
skipping occurrences or returning false entailment/consistency. Newly constructed
terms must not trigger unbounded recursive occurrence/construction feedback.
These rules do not determine a complete controller schedule for Appendix A's
postulated result or certify full RDFS execution.

Test-first fixtures independently enumerate one bounded nested root and apply
the actual checked definitions. Cases include nested predicates, literal and
triple-term resource typing, metadata exclusion, repeated scoped blanks, no
quoted assertions, target-resource-only selection, relation forgery isolation
and checked nested JSON/Postcard rejecting unsafe heads. The fixture interpreter
does not implement production async traversal, scoping, storage or matching;
consumer eviction/reload/error and entailment tests remain mandatory separately.

Sources:
- https://www.w3.org/TR/2026/CRD-rdf12-semantics-20260924/
  sections 8.1.1, 9.2.1 and Appendix A.
- https://www.w3.org/TR/2026/CRD-rdf12-concepts-20261116/#section-rdf-terms
  section 3.2 (appearing terms/triples); the dated snapshot fixes the reference
  reached through the semantics document's Concepts link.
