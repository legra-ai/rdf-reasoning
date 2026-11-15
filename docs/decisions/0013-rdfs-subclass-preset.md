# ADR 0013: Exact subclass preset for the first portable engine slice

The inference-engine consumer initially executes membership inheritance and
subclass transitivity using shared RuleId::Rdfs9 and RuleId::Rdfs11 definitions.
Expose that exact selection as Preset::RdfsSubclass, serialized and parsed as
rdfs-subclass. Do not broaden it to the fourteen-clause rdfs-core contract or
make a full RDFS claim. Existing presets keep their meaning.

Compile the same two checked definitions in deterministic order. New explicit
OtherRdfsRules and OwlRules exclusions identify the remaining structural and
OWL clauses; retain the specific axiom/datatype/container/simple-entailment,
equality/list/cardinality/inconsistency exclusions. The engine independently
admits and implements the required tuple domain and execution capabilities.
This crate supplies no execution, graph scope, provenance or storage policy.

Tests first demonstrate the missing selector. Independent bounded fixed-point
fixtures cover transitive membership, subclass cycles, disconnected classes,
unrelated properties, absent Resource/domain consequences and empty input.
Nested JSON and Postcard preserve exact selection and exclusions; unknown names
and malformed selection objects fail. Existing generic tests validate compiled
programs and checked transport for every preset.

New public enum variants require downstream exhaustive matches to be updated;
mark the change breaking and apply the authorized minor-release policy.
