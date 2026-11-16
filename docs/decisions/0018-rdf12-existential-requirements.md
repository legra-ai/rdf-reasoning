# ADR 0018: Finite RDF 1.2 nonempty-class descriptors

The public engine and Datalog model owners agreed a semantic descriptor boundary
for rdfD1a and rdfs14a. Rdf12ExistentialAxiomId has exactly four instances: the
three RdfD1a default datatype parameters and Rdfs14a. The parameter uses the
existing checked Rdf12Datatype identity. name() and class_iri() identify the
clause and required class. Nested JSON/Postcard reject unsupported parameters.
Unknown encountered types and graph datatype declarations cannot extend this map.

These describe nonempty classes even for an empty source, independently of
whether any corresponding literal or triple term occurs. RDF selects the three
datatype requirements; RDFS additionally selects the proposition requirement.
They are not ordinary safe Rule/Program values with unbound head variables.
No generic existential language, witness allocator or executor is added here.

The caller allocates a fresh scoped blank once per descriptor and immutable
attempt/snapshot, persisting the binding in its own storage for idempotent reuse.
Do not remint on each fixed-point iteration, use a global cache, Skolem IRI,
fixed user IRI or a chosen canonical literal. Different syntactic witness keys
do not assert different denotations or a unique-name assumption. Evidence owns
the exact descriptor, ground membership tuple, semantic scope and input origin;
do not fabricate an RDF premise for an unconditional semantic requirement.

The library's test-first fixtures prove the exact finite inventory, class and
clause identities, parameter-preserving nested transport and invalid-input
rejection. Executors separately must prove empty-source existential targets,
rejection of an unsupported fixed-user-IRI conclusion, scope separation and
idempotence, with bounded matching and explicit resource/I/O failures. A model
round trip is not a certificate that any such execution occurred. Existing
presets remain restricted; complete RDF/RDFS 1.2 is not claimed.

Source: https://www.w3.org/TR/2026/CRD-rdf12-semantics-20260924/
sections 8.1.1 (rdfD1a) and 9.2.1 (rdfs14a).
