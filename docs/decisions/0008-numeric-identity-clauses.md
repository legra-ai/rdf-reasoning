# ADR 0008: Share restricted numeric dt-eq and dt-diff clauses

Accepted for interpreted decimal/float/double values.

OWL-RL Table 8 defines equality and difference consequences by literal data
values, not source term inequality or IEEE arithmetic equality. Provide two
checked clauses identified as numeric/dt-eq and numeric/dt-diff. These are
restricted domain identities; do not serialize them under unqualified complete
datatype rule names or add them implicitly to existing presets.

A separate OWL_NUMERIC_VALUE_RELATION binds exact source literal identity to
one canonical semantic key per interpreted numeric value. Keys preserve exactly
OwlNumericValue identity, including disjoint spaces, signed zeros, infinities
and one NaN in each floating space. The adapter must preserve key injectivity,
validate lexical forms before interpreting values and supply the complete
numeric literal domain for its selected graph scope. Hash collisions, an
unknown-value sentinel, source lexical identity and compatibility mathematical
keys cannot implement this contract. Serde bytes are transport, not a promised
canonical semantic identity encoding.

Equality joins two literals to one key. Difference joins each literal to its
key and applies structural Ne to the bound keys. Emit generalized triples with
owl:sameAs and owl:differentFrom respectively, in both directions. Keep equality
self-pairs, including NaN; never derive self-difference from its IEEE comparison.
Neither clause needs a triple assertion anchor once literal-domain facts exist.
Unrelated triples and the compatibility value relation cannot seed this domain.

The adapter owns execution, indexes, graph scope and resource policy. It must
reject unsupported capabilities or interpretation rather than silently weaken
consequences. Do not materialize all pairs in this library. Native forward or
backward kernels may implement equivalent indexed queries; output size can be
quadratic and standard RDF projection must happen after generalized reasoning.

Tests independently evaluate the emitted clauses on tiny bounded relations.
Hard-coded expected outcomes cover decimal synonyms, disjoint numeric spaces,
signed zeros, infinities and NaN payloads; both ordered pairs and self-pairs
are checked. JSON/Postcard tests cover identifiers and checked definitions.

This does not implement a lexical parser, other literal domains, datatype
membership rules, inconsistency detection, a complete OWL-RL preset, consumer
integration or a production executor. Existing compatibility contracts remain
separate.

Normative rule source: OWL 2 Profiles, second edition, Table 8:
https://www.w3.org/TR/2012/REC-owl2-profiles-20121211/#Reasoning_in_OWL_2_RL_and_RDF_Graphs
Value identity follows ADR 0007 and OWL 2 Syntax sections 4.1–4.2.
