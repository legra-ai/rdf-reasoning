# ADR 0003: Version reference quirks separately from standard semantics

Accepted for the owlrl 7.6.2 flat schema differences.

Standard relational clauses retain their complete generalized consequences.
The Python reference's schema routine suppresses selected reflexive conclusions
and additionally treats rdf:Property alongside object/datatype properties for
self-equivalence. These choices must not redefine standard rule identities.

Expose eight Owlrl762RuleId clauses with versioned Serde identities. Reconstruct
them from the standard catalogue, using exact-term Ne guards or the additional
property-equivalence conclusion. Validate every resulting rule with the shared
Datalog constructor. Ne compares structural term identity; it does not implement
datatype-value inequality or invoke Python conversion code.

Tests distinguish standard reflexivity from the reference guards, preserve
nonreflexive consequences and round-trip every identifier and checked rule in
JSON and Postcard. Unknown and unversioned identities fail. These clauses model
relational consequences, not witness multiplicity or arbitrary Python callback
ordering, and do not constitute complete reference compatibility.

The selected standard presets do not include these clauses. An adapter must
select compatibility behavior deliberately and must implement its exact filter
operations before accepting the rule. Storage, round scheduling and Python API
lifecycle remain separate contracts.

Reference evidence: owlrl 7.6.2 OWLRL_Semantics._schema_vocabulary, independently
compared with the standard clauses; no reference implementation was copied.
