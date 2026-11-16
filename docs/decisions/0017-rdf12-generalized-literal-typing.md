# ADR 0017: Explicit default-map generalized literal typing

The inference-engine owner agreed to a virtual metadata relation with distinct
semantic evidence for GrdfD1. Publish checked definitions independently of
execution: Rdf12LiteralRuleId::GrdfD1(Rdf12Datatype) has exactly three default-map
instances. Identity is the clause name plus the exact datatype parameter.
Externally tagged checked Serde preserves that parameter as its full IRI.

Each definition has one positive auxiliary premise:
RDF12_LITERAL_DATATYPE_RELATION(literal, datatype). The head is the generalized
triple (literal, rdf:type, datatype). Fix the selected datatype as a constant in
both positions. Do not use a user graph's rdf:type rdfs:Datatype statements as
recognition configuration; opaque encountered datatypes cannot match these
default-map instances. No existing preset changes its selection.

The adapter supplies exact literal identity and declared datatype metadata after
standard facet and recognized lexical validation. It binds only literals from
the selected immutable source, target-context or recursive occurrence scope.
Invalid recognized literals must not receive guessed positive premises. The
same definition applies to each explicit semantic phase; it does not choose
which phase is active or conflate impossible targets with inconsistent sources.

The auxiliary premises are not RDF assertions. Keep literal occurrence, metadata
validation and source/target origin as semantic evidence; never manufacture RDF
triple witnesses or expose metadata tuples as user RDF. A consumer must explicitly
admit and test this virtual relation. Ordinary positive Datalog syntax support
does not certify relation binding, lexical inspection or actual execution.

Only three finite clauses are allocated, with two premise terms and three head
terms each. No literals, lexical strings or graphs are read or retained by this
library. Forward and backward adapters may bind the same relation through their
own streaming/indexed capabilities. Preserve generalized consequences until the
appropriate output projection, without whole-graph conversion.

Independent test-first fixtures cover all instances, exact keys, datatype
mismatches, missing metadata, opaque types despite graph declarations, unrelated
relations and source/target/nested context keys. Nested JSON/Postcard exercises
checked programs and rejects unsafe heads and unsupported datatype identities.
Whole consistency, existential witnesses, proposition/triple-term rules and
complete target entailment remain separate contracts.

Source: https://www.w3.org/TR/2026/CRD-rdf12-semantics-20260924/#entailment-rules
