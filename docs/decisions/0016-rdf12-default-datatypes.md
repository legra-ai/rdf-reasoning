# ADR 0016: RDF 1.2 default datatype recognition and lexical validity

The pinned RDF 1.2 Semantics CRD 2026-09-24 requires xsd:string, rdf:langString
and rdf:dirLangString in every RDF interpretation's recognized datatype map.
Expose those exact identities as Rdf12Datatype, including ALL, iri, encountered
IRI recognition and strict requested-IRI parsing. Serialize the full datatype
IRI. Unknown encountered datatypes remain opaque under the default map;
unsupported requested recognition fails rather than being ignored or guessed.
This API validates one requested identity, not an arbitrary configuration's
completeness. Consumers must retain all three mandatory default datatypes.

Share pure lexical validation in rdf-reasoning so the public executor and Legra's
separate forward/backward adapters reuse one semantic predicate. No public
executor, Oxigraph dependency or private lifecycle/storage change is implied.
validate_lexical borrows a UTF-8 string or fragment. XML1.1 Char applies to
xsd:string: nonzero C0 characters are permitted; NUL, U+FFFE and U+FFFF are not.
Rust str already admits only Unicode scalar values. No XML1.0 compatibility
validator, normalization, conversion key or source-term replacement is added.

Syntactically legal language and directional strings have no ill-typed lexical
forms. Structural metadata must first validate standard datatype/facet pairing;
language validity, ASCII case-folded tag identity and direction belong to the
shared RDF model. Do not duplicate its parser or reject those payloads using
the xsd:string restriction. Unchecked extended metadata is not a standard literal.

Streaming adapters own UTF-8 decoding, offset accumulation, source identity and
source/target role policy. The semantic predicate needs no graph, hydration or
stored lexical payload. Invalid xsd:string returns the first character and a
u64 fragment-relative byte offset. The diagnostic's checked JSON/Postcard
constructor rejects characters which actually satisfy XML1.1. An offset does
not prove an external literal's content or the presence of a graph assertion.

This is a lexical building block, not whole-graph consistency. Recognized class
membership/disjointness constraints, GrdfD1, nonempty-datatype witnesses, target
semantics and existential entailment checking remain required. Inconsistent
sources and impossible targets require their own explicit consumer policy;
these helpers do not return a false consistency verdict from missing evidence.

Regression tests precede implementation. Independent Unicode boundary fixtures
distinguish XML1.1 from XML1.0; language/directional payloads remain unrestricted.
Tests preserve first-error UTF-8 byte positions, strict recognition distinctions,
full-IRI identities and nested JSON/Postcard with invalid diagnostic rejection.

Source: https://www.w3.org/TR/2026/CRD-rdf12-semantics-20260924/#D_interpretations
