# ADR 0020: Positive default-map datatype conflicts

The engine and Datalog model owners agreed exactly three canonical unordered
pairs: xsd:string/rdf:langString, xsd:string/rdf:dirLangString, and
rdf:langString/rdf:dirLangString. The pinned RDF 1.2 semantics assigns distinct
string, language-pair and directional-triple value spaces; RDF section 8 and
RDFS section 9 fix their rdf:type class extensions. One member in two of these
classes is therefore inconsistent. Do not generalize this to arbitrary datatype
IRIs or derive recognition from user datatype declarations.

Rdf12DatatypeConflictId enumerates those pairs with ALL, name, datatypes,
witness_relation and definition. Names are implementation diagnostics, not
invented W3C rule labels. Each checked positive Rule has two fixed rdf:type
premises and a dedicated unary non-RDF diagnostic head projecting the exact
shared member key. Both class IRIs are constants; different member keys are
not guessed equal from lexical resemblance or a unique-name assumption.
No missing premise proves nonmembership. No denial IR or preset is added.

The executor checks the selected source closure, including source-scoped
nonempty datatype witnesses and source-derived memberships, before introducing
target-context premises. Preserve both exact ground memberships, diagnostic ID,
scope and derivation evidence. Target-local context must not contaminate source
consistency. Lexical errors and unsupported recognition retain their separate
source/target policies. Do not manufacture an RDF assertion for a diagnostic.

Both asserted and inferred memberships may reside outside memory. Executors
must probe their authoritative async source/store and reload evicted facts;
cache misses do not establish absence. Bounded joins, cancellation, read errors
and budget exhaustion remain explicit, never a false consistent/not-entailed
answer. This library allocates only finite rule metadata and owns no store,
Tokio runtime, graph collection or execution state. Consumers must separately
test eviction/reload and failure behavior across their own execution strategies.

Test-first fixtures cover all three pairs and premise order, exact shared keys,
missing/different members, unknown datatypes despite declarations, user RDF
using a diagnostic IRI, and checked nested JSON/Postcard with unsafe heads
rejected. A no-literal-data subclass fixture combines the shared rdfD1a class
descriptor with rdfs9 and the conflict clause. Witness allocation and complete
consistency/execution certification remain separate consumer obligations.

Source: https://www.w3.org/TR/2026/CRD-rdf12-semantics-20260924/
sections 7, 8 and 9.
