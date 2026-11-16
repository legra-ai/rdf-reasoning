# ADR 0019: Explicit generalized proposition constructor

The public engine and Datalog owners agreed the Grdfs14 definition and canonical
three-slot RDF relation encoding. Rdf12PropositionRuleId::Grdfs14 supplies one
checked Rule, ALL and its serialized name grdfs14. Three ordinary premises bind
the property, subject resource and object resource. The head types a TripleTerm
as rdfs:Proposition. Its inner Atom uses TRIPLE_RELATION and the ordered terms
subject, predicate and object. The inner relation symbol is not an RDF predicate;
do not convert this to the alternative fixed-predicate, two-operand encoding.

Generalized intermediate terms must survive binding. In particular a literal
may acquire rdf:Property membership through range reasoning and then bind the
predicate slot. Do not silently reject it by routing construction through a
strict standard-RDF Triple constructor. Nested triple terms remain terms; their
contents do not become asserted facts. Standard RDF model validation is unchanged.

This is a sound constructor clause, not a complete execution schedule. Require
explicit runtime support and a finite construction phase over a selected
immutable snapshot. Do not put it into existing presets or allow unrestricted
recursive constructor fixed points. The controller must supply adequate typing
and source/target/substitution vocabulary. Initial source snapshots alone can
lack prerequisites discovered through recursive occurrence or core rules.
The pinned draft postulates Appendix A completeness; the catalogue neither
proves that result nor chooses a complete controller schedule.

Retain the exact rule ID, all three ground premise tuples, phase/snapshot and
input origin as evidence. Runtime substitution must check nested-term limits:
the declared Datalog model admits depth four, so wrapping an admitted depth-four
term can exceed the limit. Fail explicitly; do not drop that conclusion or turn
a resource error into a negative entailment result. Template validation alone
does not certify ground output or unlimited RDF 1.2 input depth.

Test-first fixtures run against the actual declared Datalog 3.0.4 dependency.
A bounded independent single-phase interpreter checks ordered construction,
generalized predicate keys, nested objects, missing premises, no quoted-content
assertion and explicit nesting failure. Checked JSON/Postcard preserve the rule
and reject unbound nested variables. Positive flat admission rejects the clause;
explicit TripleTerms capability admits its syntax. Consumer execution and full
RDFS conformance require separate tests; occurrence substitution rdfs14 remains
outside this slice.

Source: https://www.w3.org/TR/2026/CRD-rdf12-semantics-20260924/
Appendix A, Grdfs14.
