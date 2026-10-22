# ADR 0004: Model reachable list membership as an auxiliary relation

Accepted for the positive list catalogue.

List-based rules must keep the declaration joined to its own list. Define a
portable two-place reachable-member relation through direct first edges and
recursive rest edges, then express cls-int2, cls-uni, cls-oo, scm-int and scm-uni
as checked Datalog clauses over that relation and generalized triples.

Keep ListRuleId::ALL distinct from the flat RuleId::ALL: adapters restricted to
three-place triple atoms cannot silently execute auxiliary relations. They must
reject unsupported clauses or provide exactly the declared relation, for
example through indexed traversal or an equivalent native property path.
Storage, relation indexing and fixed-point execution remain caller-owned.

Reachability does not certify list well-formedness. Cycles have a finite least
fixed point on finite input; dangling and branching edges preserve the literal
relation definition. Validation and profile-specific errors remain explicit
caller contracts. No graph data or backend is introduced in this crate.

The five consequences do not cover universal intersection recognition, chains,
keys or every schema list condition. Existing preset inventories and exclusions
remain unchanged. Independent finite consequence, cross-list, cycle and Serde
JSON/Postcard fixtures guard the published definitions.
