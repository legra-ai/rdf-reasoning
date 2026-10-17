# rdf-reasoning

[![Crates.io](https://img.shields.io/crates/v/rdf-reasoning.svg)](https://crates.io/crates/rdf-reasoning)
[![Documentation](https://docs.rs/rdf-reasoning/badge.svg)](https://docs.rs/rdf-reasoning)
[![Downloads](https://img.shields.io/crates/d/rdf-reasoning.svg)](https://crates.io/crates/rdf-reasoning)
[![CI](https://github.com/legra-ai/rdf-reasoning/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/rdf-reasoning/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE-MIT)

Portable RDF entailment rule definitions, compiled to
[`datalog-structural-model`](https://github.com/legra-ai/datalog-structural-model).
The name means **reasoning over RDF graphs**. RDF supplies the shared data model
and baseline semantics; RDFS and OWL add schema and ontology semantics. This
repository groups reusable RDFS and RDF-based OWL entailment rules and presets
under that common graph foundation, including the planned Horst variants.

Each preset identifies its own rules and exclusions. Keeping them together
allows consumers to reuse one catalogue while selecting the semantics they
support. RDF data types live in
[`rdf-structural-model`](https://github.com/legra-ai/rdf-structural-model), and
the generic rule representation lives in `datalog-structural-model`.

```rust
use rdf_reasoning::Preset;
use datalog_structural_model::DatalogProfile;

let preset: Preset = "rdfs-plus-core".parse()?;
let program = preset.program();
program.validate_for(&DatalogProfile::positive())?;
assert_eq!(program.rules.len(), 18);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Exact implemented presets

| Selector | Rules |
| --- | --- |
| `none` | No rules or axioms |
| `rdfs-core` | `rdf1`, `rdfs2`, `rdfs3`, `rdfs4a`, `rdfs4b`, `rdfs5`–`rdfs13` |
| `rdfs-plus-core` | `rdfs-core` plus `prp-symp`, `prp-trp`, `prp-inv1`, `prp-inv2` |

These are explicitly restricted rule presets. Both core presets omit axiomatic
triples, datatype recognition/value reasoning/ill-typed literal checks, automatic
`rdf:_n` classification, existential simple-entailment checking and all equality
rules. `Preset::rules()` and `Preset::exclusions()` expose the exact contract.
`RuleId::definition()` returns each actual checked clause, rather than a label
that an engine must independently implement.

`rdfs`, `owl2-rl`, `owl-horst` and vendor names are rejected. Full RDFS, OWL 2
RL/RDF, precisely defined Horst variants and Python reference closure options
remain further work. Neither core preset is a GraphDB compatibility claim;
`rdfs-plus-core` specifies just the listed property rules. No standard profile
name is accepted as an alias for a restricted implementation.

The RDFS structural clauses follow the entailment patterns in
[RDF 1.1 Semantics](https://www.w3.org/TR/rdf11-mt/#patterns-of-rdfs-entailment).
`rdf1` is the conventional historical identifier for predicate/property typing.
The four additional property clauses follow
[OWL 2 Profiles, second edition, Table 5](https://www.w3.org/TR/owl2-profiles/#Reasoning_in_OWL_2_RL_and_RDF_Graphs).
The implementations and tiny entailment fixtures were independently authored;
no private engine, vendor rule files or third-party implementation was copied.

## Adapter contract

Every clause uses the three-place `TRIPLE_RELATION` relation in subject,
predicate, object order. Rule constants contain full RDF/RDFS/OWL vocabulary
IRIs. An adapter must bind those constants as IRIs and preserve exact identity
for all other terms, including lexical literals and scoped blank nodes. These
clauses do not inspect literal values or the contents of RDF 1.2 triple terms;
loading a newer RDF syntax does not enable extra entailment rules.

Intermediate tuples are generalized: literal subjects and non-IRI predicates
can participate in derivations. Filtering them out before closure can remove
valid consequences. Standard RDF output projection is a separate adapter step.
No `owl:sameAs` expansion or representative compression is implied.

Forward and backward execution, graph selection, named-graph scope, result
destination and consistency policy belong to the caller. The crate neither
reads a graph nor owns storage or an evaluator. A caller must explicitly bind
its selected graph scope to the triple relation and independently establish
that its engine implements the required Datalog operations and RDF term domain.
The generated program's `required_capabilities()` describes language syntax;
it does not certify those adapter semantics.

Only the finite rule catalogue is allocated (at most 18 clauses). There is no
graph-sized collection, filesystem access, backend dependency or mutable global.
The catalogue can serve an asynchronous streaming or disk-backed executor
without collecting its dataset.

## Serialization and validation

All exported semantic identifiers, exclusions and diagnostics support Serde.
Exact kebab-case preset names are shared by string parsing and Serde. Unknown
names, variants and diagnostic fields fail; there is no disabled/empty fallback.
JSON and Postcard round trips exercise the portable representation, including
compiled Datalog programs. Derived rule content is reconstructed from the
selected preset, so a serialized preset cannot silently change its clauses.

Program and rule constructors use the Datalog model's checked construction.
Invalid built-in definitions are programming errors and panic during assembly.
Allocation and decoder budgets for caller-supplied data remain caller-owned.
Serde bytes are not an RDF syntax or a canonical identity encoding.

## Development

Rust 1.99.1; formatting uses the separate pin in `rustfmt-toolchain`.

```sh
cargo test --locked
python3 scripts/test-package-toolchain.py
python3 scripts/test-package.py
python3 scripts/test-commit-policy.py
./scripts/fmt.sh --check
cargo clippy --all-targets --all-features -- -D warnings
```

The shared main CI pipeline is the only publisher. PRs validate the checkout
and packaged tests; Linux and Windows gate publication from the tested source.
Repository protection, publication credentials and automatic-release evidence
must be verified separately during bootstrap.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
