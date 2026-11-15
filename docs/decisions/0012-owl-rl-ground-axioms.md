# ADR 0012: Explicit finite OWL-RL ground axioms

Expose the four empty-body rule families in OWL 2 Profiles Second Edition
(2012), section 4.3: prp-ap, cls-thing, cls-nothing1 and dt-type1. Use checked
Datalog facts over the existing generalized triple relation. No separate axiom
executor or graph representation is needed. Return at most32 ground clauses
per family; all vocabulary is fixed independently of the input graph.

Keep selection explicit through OwlRlAxiomId, separate from existing presets.
Nine built-in annotation property declarations and Thing/Nothing class
declarations are unconditional; declaring Nothing as a class is not asserting
that any individual belongs to Nothing. The32 datatype declarations are the
exact map in section4.2.1. They neither interpret literal values nor certify a
complete datatype reasoner. Exclude owl:real/owl:rational and do not silently
substitute newer RDF vocabulary for rdf:PlainLiteral. Optional RDF/RDFS axioms
remain separate work.

Independent finite inventory tests fail before implementation, then validate
groundness, exact declarations, unwanted-vocabulary absence and unchanged
presets. Identifiers and checked clauses round-trip through JSON and Postcard;
unknown identifiers and malformed shapes fail. Existing graph scope, execution,
atomic publication and capability admission remain consumer responsibilities.

Sources:
- https://www.w3.org/TR/2012/REC-owl2-profiles-20121211/#Reasoning_in_OWL_2_RL_and_RDF_Graphs
- https://www.w3.org/TR/2012/REC-owl2-profiles-20121211/#Entities_3
- https://www.w3.org/TR/2012/REC-owl2-syntax-20121211/#Annotation_Properties
