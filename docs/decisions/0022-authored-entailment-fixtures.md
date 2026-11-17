# ADR 0022: Portable authored entailment regression data

The engine and Datalog owners agreed a small typed JSON fixture manifest, rather
than a Rust-only oracle or another shared evaluator. The data fixes semantic and
Turtle syntax editions, format 1, a positive fixture revision, dual license and
authored-regression provenance. These cases are independently authored project
regressions, not official W3C cases, normative assertions or complete conformance
certification. No new semantic API, parser, runtime or model dependency is added.

Each stable case ID owns explicit source/target Turtle strings, an expected
Boolean entailment and required semantic capabilities. Source and target parse
in independent blank scopes. Repeated/nested labels within one document retain
identity; distinct target blanks need not map injectively. Explicit prefixes
keep each document self-contained. The default-map RDFS regime is fixed at the
manifest root; requirement tags identify controller operations used by a case,
not an executor certificate inferred from a program or a complete profile alias.

Test-only typed Serde models deny unknown fields and requirement values, reject
duplicate IDs/requirements, enforce pinned identity and finite case/document
bounds, and round-trip nested JSON/Postcard. They validate the data contract,
not Turtle syntax or entailment. Actual public/private consumer adapters must
parse and execute the cases independently. Unsupported requirements are explicit
unsupported outcomes, never skipped passes; parse, I/O, cancellation and budget
errors are not negative entailment answers. Every adapter must separately test
asserted/inferred fact eviction and reloading through authoritative async access.

The published crate includes fixture JSON. Existing packaged validation checks
tracked fixture presence and exact bytes before extracting, then runs Rust tests
whose include_str reads that extracted data. A tooling regression rejects missing
and altered archive payloads and accepts exact payloads. This does not introduce
a second CI/release pipeline or require a shared parser in the rule catalogue.

The initial fifteen cases cover triple-term abstraction through the composition
of occurrence typing, proposition construction and consistent simple matching;
repeated, conflicting and noninjective mappings; mixed nested/outer abstraction;
blank-label scope collisions; no quoted assertions; nested predicate typing;
source-only and target-only container vocabulary; target-context limits; and
nonempty datatype witnesses without fixed-user-IRI consequences. Engine reports
must distinguish case execution from catalogue validation. Complete controller
scheduling and full RDFS certification remain separate work.

Sources:
- https://www.w3.org/TR/2026/CRD-rdf12-semantics-20260924/
- https://www.w3.org/TR/2026/WD-rdf12-turtle-20261111/
