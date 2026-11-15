# ADR 0015: Explicit finite RDF 1.1 vocabulary seeds

The inference-engine consumer needs full RDFS, not a relabeling of the existing
fourteen-rule core. Pin this slice to RDF 1.1 Semantics Recommendation (2014).
Supply the exact non-container RDF and RDFS ground axioms as independent checked
Datalog facts: eight and thirty-eight respectively. A third family declares only
the default recognized datatypes, xsd:string and rdf:langString, as rdfs:Datatype.
These two declarations supply the default-map instances of rdfs1. Do not
recognize other datatypes merely because they occur in input.

The infinite container vocabulary must never be enumerated or stored in this
library. Rdf11ContainerProperty validates one caller-owned or borrowed absolute
IRI. Require the exact RDF namespace followed by an underscore and a canonical
positive ASCII decimal suffix. Do not parse that suffix into a machine integer,
normalize leading zeros, accept Unicode digits or substitute a fallback on error.
Its fixed outputs are one RDF property declaration and three RDFS declarations
for membership-property type, domain and range. Checked JSON/Postcard decoding
uses the same constructor; unknown fields and invalid names fail.

Selection is an executor/entailment contract. Appendix A closure towards a target
uses that target's container vocabulary and adds the rdf:_1 witness when no
container axioms were selected. Looking only at observed source predicates is
insufficient; target-only vocabulary and IRIs in other positions matter. Finite
vocabulary materialization and complete source-to-target entailment are distinct
operations. The seed API neither chooses the target nor owns a graph collection.

Only fixed metadata is collected: at most thirty-eight rules per family and four
rules per selected container IRI. Source traversal, deduplication, bounded scratch,
graph/blank-node scope, atomic publication and target matching belong to consumers.
No preset changes, executor, backend or new dependency are introduced here.

This is not full RDFS delivery. Default-map lexical validation, GrdfD1 generalized
literal typing, consistency checks for recognized value spaces and existential
simple-entailment matching remain required. rdfD2 already has the existing Rdf1
definition. Full recognized datatype configurations must be explicit; unknown
encountered datatype literals remain opaque, while an unsupported requested
recognition map must fail. RDF 1.2 draft extensions are not silently enabled.

Independent inventory, malformed-IRI, large-suffix, borrowing and checked nested
transport fixtures precede implementation. Existing profiles retain their exact
rules and exclusions.

Sources:
- https://www.w3.org/TR/2014/REC-rdf11-mt-20140225/#rdf-interpretations
- https://www.w3.org/TR/2014/REC-rdf11-mt-20140225/#rdfs-interpretations
- https://www.w3.org/TR/2014/REC-rdf11-mt-20140225/#entailment-rules
- https://www.w3.org/TR/2014/REC-rdf-schema-20140225/#ch_containermembershipproperty
