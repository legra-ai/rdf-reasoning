//! Catalogue programs compose with corrected shared Datalog validation.

use datalog_structural_model::{
    AggregateBody, Aggregator, Atom, BindExpr, ConstantSymbol, Literal, PredicateSymbol, Rule,
    Term, VariableSymbol,
};
use rdf_reasoning::Preset;

fn atom(predicate: &str, term: Term) -> Atom {
    Atom {
        predicate: PredicateSymbol::new(predicate),
        terms: vec![term],
    }
}

#[test]
fn catalogue_program_accepts_aggregate_exports_in_a_later_binding() {
    let total = VariableSymbol::new("total");
    let copy = VariableSymbol::new("copy");
    let input = VariableSymbol::new("input");
    for preset in Preset::ALL {
        // bounded: one extension of each finite built-in rule catalogue.
        let mut program = preset.program();
        program.rules.push(Rule::new(
            atom("urn:test:output", Term::Variable(copy.clone())),
            vec![
                Literal::Aggregate(AggregateBody {
                    result_var: total.clone(),
                    aggregator: Aggregator::Count,
                    input_var: input.clone(),
                    group_by: vec![],
                    inner_body: vec![Literal::Positive(atom(
                        "urn:test:input",
                        Term::Variable(input.clone()),
                    ))],
                }),
                Literal::Bind {
                    var: copy.clone(),
                    expr: BindExpr::Term(Term::Variable(total.clone())),
                },
            ],
        ));
        program.validate().unwrap();
        let json = serde_json::to_string(&program).unwrap();
        assert_eq!(
            serde_json::from_str::<datalog_structural_model::Program>(&json).unwrap(),
            program
        );
        let bytes = postcard::to_allocvec(&program).unwrap();
        assert_eq!(
            postcard::from_bytes::<datalog_structural_model::Program>(&bytes).unwrap(),
            program
        );
    }
}

#[test]
fn catalogue_extension_preserves_strict_negative_dependencies() {
    let constant = |value| Term::Constant(ConstantSymbol::new(value));
    let mut program = Preset::RdfsPlusCore.program();
    let offset = program.rules.len();
    // bounded: three additional authored clauses, independent of graph size.
    program.rules.extend([
        Rule::positive(atom("urn:test:p", constant("a")), vec![]),
        Rule::new(
            atom("urn:test:p", constant("b")),
            vec![Literal::Negative(atom("urn:test:excluded", constant("b")))],
        ),
        Rule::new(
            atom("urn:test:q", constant("a")),
            vec![Literal::Negative(atom("urn:test:p", constant("a")))],
        ),
    ]);
    let strata = program.stratify().unwrap();
    assert!(strata.stratum_of_rule(offset + 2).unwrap() > strata.stratum_of_rule(offset).unwrap());
}

#[test]
fn catalogue_extensions_allow_stratified_tuple_partitions() {
    for preset in Preset::ALL {
        let mut program = preset.program();
        let constant = |value| Term::Constant(ConstantSymbol::new(value));
        program.rules.extend([
            Rule::new(
                atom("urn:test:partition", constant("b")),
                vec![Literal::Negative(atom("urn:test:partition", constant("a")))],
            ),
            Rule::positive(atom("urn:test:partition", constant("a")), vec![]),
        ]);
        program.validate().unwrap();
        let strata = program.stratify().unwrap();
        let offset = program.rules.len() - 2;
        assert_eq!(strata.stratum_of_rule(offset), Some(1));
        assert_eq!(strata.stratum_of_rule(offset + 1), Some(0));
        let json = serde_json::to_string(&program).unwrap();
        assert_eq!(
            serde_json::from_str::<datalog_structural_model::Program>(&json).unwrap(),
            program
        );
        let bytes = postcard::to_allocvec(&program).unwrap();
        assert_eq!(
            postcard::from_bytes::<datalog_structural_model::Program>(&bytes).unwrap(),
            program
        );
    }
}
