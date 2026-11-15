# ADR 0015: Explicit finite RDF 1.2 vocabulary seeds

The user clarified the compliance target as RDF 1.2/RDFS 1.2 before this slice
was published. Replace the unpublished RDF 1.1 proposal directly. Expose only
Rdf12AxiomId and Rdf12ContainerProperty; no compatibility aliases or parallel
implementation are introduced. Existing restricted presets remain explicit
subsets; no complete RDFS selector is enabled by this seed catalogue.

Pin RDF 1.2 Semantics CRD 2026-09-24 and Schema WD 2026-11-12. Supply exact
non-container ground axioms through checked Datalog facts: nine RDF and forty-one
RDFS declarations. These include rdf:reifies property/domain/range and the
rdfs:Proposition subclass axiom. A third family declares the mandatory default
datatypes xsd:string, rdf:langString and rdf:dirLangString as rdfs:Datatype. This
supplies default-map rdfs1 instances, not literal interpretation or recognition
of other encountered datatype IRIs.

Never enumerate the infinite container vocabulary or collect caller graphs.
Rdf12ContainerProperty checks one owned or borrowed absolute IRI: the exact RDF
namespace followed by underscore and positive ASCII decimal digits without
leading zeros. Do not parse the suffix into a machine integer, normalize invalid
names, accept Unicode digits or substitute a fallback on error. Methods return
one RDF property declaration and three RDFS declarations for membership-property
type, domain and range. Checked nested JSON/Postcard uses the same constructor;
unknown fields and invalid names fail. Proof identity is family plus exact
ground triple; no unstable catalogue ordinal is promised.

Selection belongs to the executor. Appendix A closure towards a target uses
target-relevant container vocabulary and the rdf:_1 witness when none was
selected. Observed source predicates alone are insufficient. Target-context
literal/type/resource rules and recursively appearing terms are additional
obligations. Finite vocabulary materialization and source-to-target existential
entailment are distinct operations. The draft postulates rather than proves its
closure completeness result; conformance requires normative conditions and
independent official tests, not a stronger theorem claim.

Only fixed metadata is allocated here: at most forty-one clauses per family and
four per selected container IRI. Consumers own source traversal, deduplication,
bounded scratch, blank-node/graph scope, target matching and atomic publication.
No executor, backend, dependency or preset is added. A bounded test interpreter
admits up to4096 facts for the complete fixed-vocabulary regression; previous
tiny fixtures retain their128-fact limit.

Full RDFS 1.2 remains incomplete. Required follow-on contracts include XML1.1
Char validation for xsd:string, language and directional value identity, GrdfD1,
rdfD1a nonempty-datatype witnesses, rdfs14/rdfs14a/Grdfs14 proposition behavior,
triple-term substitutions, recognized-value-space consistency and existential
target matching. The existing Rdf1 clause supplies rdfD2. Unknown encountered
datatypes remain opaque; unsupported requested recognition maps must fail.

Tests first cover missing APIs and exact revised inventories, malformed and
arbitrary-length suffixes, borrowing and checked nested transport. Independent
empty-source fixed-point fixtures prove selected target-only container seeds
join the inherited core clauses. No full RDFS execution claim follows.

Sources:
- https://www.w3.org/TR/2026/CRD-rdf12-semantics-20260924/
- https://www.w3.org/TR/2026/WD-rdf12-schema-20261112/
