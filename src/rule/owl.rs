//! Positive flat OWL clauses; graph execution and projection remain caller-owned.

use super::{Pattern, RuleId};
use datalog_structural_model::Rule;

impl RuleId {
    pub(super) fn owl_definition(self) -> Rule {
        let (head, body): ([&str; 3], &[[&str; 3]]) = match self {
            Self::EqRefSubject => (["?s", "owl:sameAs", "?s"], &[["?s", "?p", "?o"]]),
            Self::EqRefPredicate => (["?p", "owl:sameAs", "?p"], &[["?s", "?p", "?o"]]),
            Self::EqRefObject => (["?o", "owl:sameAs", "?o"], &[["?s", "?p", "?o"]]),
            Self::EqSym => (["?y", "owl:sameAs", "?x"], &[["?x", "owl:sameAs", "?y"]]),
            Self::EqTrans => (
                ["?x", "owl:sameAs", "?z"],
                &[["?x", "owl:sameAs", "?y"], ["?y", "owl:sameAs", "?z"]],
            ),
            Self::EqRepS => (
                ["?y", "?p", "?o"],
                &[["?x", "owl:sameAs", "?y"], ["?x", "?p", "?o"]],
            ),
            Self::EqRepP => (
                ["?s", "?q", "?o"],
                &[["?p", "owl:sameAs", "?q"], ["?s", "?p", "?o"]],
            ),
            Self::EqRepO => (
                ["?s", "?p", "?y"],
                &[["?x", "owl:sameAs", "?y"], ["?s", "?p", "?x"]],
            ),
            Self::PrpFp => (
                ["?y", "owl:sameAs", "?z"],
                &[
                    ["?p", "rdf:type", "owl:FunctionalProperty"],
                    ["?x", "?p", "?y"],
                    ["?x", "?p", "?z"],
                ],
            ),
            Self::PrpIfp => (
                ["?x", "owl:sameAs", "?y"],
                &[
                    ["?p", "rdf:type", "owl:InverseFunctionalProperty"],
                    ["?x", "?p", "?z"],
                    ["?y", "?p", "?z"],
                ],
            ),
            Self::PrpEqp1 => (
                ["?x", "?q", "?y"],
                &[["?p", "owl:equivalentProperty", "?q"], ["?x", "?p", "?y"]],
            ),
            Self::PrpEqp2 => (
                ["?x", "?p", "?y"],
                &[["?p", "owl:equivalentProperty", "?q"], ["?x", "?q", "?y"]],
            ),
            Self::ClsSvf1 => (
                ["?x", "rdf:type", "?r"],
                &[
                    ["?r", "owl:onProperty", "?p"],
                    ["?r", "owl:someValuesFrom", "?c"],
                    ["?x", "?p", "?y"],
                    ["?y", "rdf:type", "?c"],
                ],
            ),
            Self::ClsSvf2 => (
                ["?x", "rdf:type", "?r"],
                &[
                    ["?r", "owl:onProperty", "?p"],
                    ["?r", "owl:someValuesFrom", "owl:Thing"],
                    ["?x", "?p", "?y"],
                ],
            ),
            Self::ClsAvf => (
                ["?y", "rdf:type", "?c"],
                &[
                    ["?r", "owl:onProperty", "?p"],
                    ["?r", "owl:allValuesFrom", "?c"],
                    ["?x", "rdf:type", "?r"],
                    ["?x", "?p", "?y"],
                ],
            ),
            Self::ClsHv1 => (
                ["?x", "?p", "?v"],
                &[
                    ["?r", "owl:onProperty", "?p"],
                    ["?r", "owl:hasValue", "?v"],
                    ["?x", "rdf:type", "?r"],
                ],
            ),
            Self::ClsHv2 => (
                ["?x", "rdf:type", "?r"],
                &[
                    ["?r", "owl:onProperty", "?p"],
                    ["?r", "owl:hasValue", "?v"],
                    ["?x", "?p", "?v"],
                ],
            ),
            Self::CaxEqc1 => (
                ["?x", "rdf:type", "?d"],
                &[
                    ["?c", "owl:equivalentClass", "?d"],
                    ["?x", "rdf:type", "?c"],
                ],
            ),
            Self::CaxEqc2 => (
                ["?x", "rdf:type", "?c"],
                &[
                    ["?c", "owl:equivalentClass", "?d"],
                    ["?x", "rdf:type", "?d"],
                ],
            ),
            Self::ScmClsSelfSubclass => (
                ["?c", "rdfs:subClassOf", "?c"],
                &[["?c", "rdf:type", "owl:Class"]],
            ),
            Self::ScmClsSelfEquivalent => (
                ["?c", "owl:equivalentClass", "?c"],
                &[["?c", "rdf:type", "owl:Class"]],
            ),
            Self::ScmClsThing => (
                ["?c", "rdfs:subClassOf", "owl:Thing"],
                &[["?c", "rdf:type", "owl:Class"]],
            ),
            Self::ScmClsNothing => (
                ["owl:Nothing", "rdfs:subClassOf", "?c"],
                &[["?c", "rdf:type", "owl:Class"]],
            ),
            Self::ScmEqc1Forward => (
                ["?c", "rdfs:subClassOf", "?d"],
                &[["?c", "owl:equivalentClass", "?d"]],
            ),
            Self::ScmEqc1Reverse => (
                ["?d", "rdfs:subClassOf", "?c"],
                &[["?c", "owl:equivalentClass", "?d"]],
            ),
            Self::ScmEqc2 => (
                ["?c", "owl:equivalentClass", "?d"],
                &[
                    ["?c", "rdfs:subClassOf", "?d"],
                    ["?d", "rdfs:subClassOf", "?c"],
                ],
            ),
            Self::ScmEqp1Forward => (
                ["?p", "rdfs:subPropertyOf", "?q"],
                &[["?p", "owl:equivalentProperty", "?q"]],
            ),
            Self::ScmEqp1Reverse => (
                ["?q", "rdfs:subPropertyOf", "?p"],
                &[["?p", "owl:equivalentProperty", "?q"]],
            ),
            Self::ScmEqp2 => (
                ["?p", "owl:equivalentProperty", "?q"],
                &[
                    ["?p", "rdfs:subPropertyOf", "?q"],
                    ["?q", "rdfs:subPropertyOf", "?p"],
                ],
            ),
            Self::ScmDom1 => (
                ["?p", "rdfs:domain", "?d"],
                &[["?p", "rdfs:domain", "?c"], ["?c", "rdfs:subClassOf", "?d"]],
            ),
            Self::ScmDom2 => (
                ["?p", "rdfs:domain", "?c"],
                &[
                    ["?q", "rdfs:domain", "?c"],
                    ["?p", "rdfs:subPropertyOf", "?q"],
                ],
            ),
            Self::ScmRng1 => (
                ["?p", "rdfs:range", "?d"],
                &[["?p", "rdfs:range", "?c"], ["?c", "rdfs:subClassOf", "?d"]],
            ),
            Self::ScmRng2 => (
                ["?p", "rdfs:range", "?c"],
                &[
                    ["?q", "rdfs:range", "?c"],
                    ["?p", "rdfs:subPropertyOf", "?q"],
                ],
            ),
            Self::ScmHv => (
                ["?r", "rdfs:subClassOf", "?s"],
                &[
                    ["?r", "owl:hasValue", "?v"],
                    ["?r", "owl:onProperty", "?p"],
                    ["?s", "owl:hasValue", "?v"],
                    ["?s", "owl:onProperty", "?q"],
                    ["?p", "rdfs:subPropertyOf", "?q"],
                ],
            ),
            Self::ScmSvf1 => (
                ["?r", "rdfs:subClassOf", "?s"],
                &[
                    ["?r", "owl:someValuesFrom", "?c"],
                    ["?r", "owl:onProperty", "?p"],
                    ["?s", "owl:someValuesFrom", "?d"],
                    ["?s", "owl:onProperty", "?p"],
                    ["?c", "rdfs:subClassOf", "?d"],
                ],
            ),
            Self::ScmSvf2 => (
                ["?r", "rdfs:subClassOf", "?s"],
                &[
                    ["?r", "owl:someValuesFrom", "?c"],
                    ["?r", "owl:onProperty", "?p"],
                    ["?s", "owl:someValuesFrom", "?c"],
                    ["?s", "owl:onProperty", "?q"],
                    ["?p", "rdfs:subPropertyOf", "?q"],
                ],
            ),
            Self::ScmAvf1 => (
                ["?r", "rdfs:subClassOf", "?s"],
                &[
                    ["?r", "owl:allValuesFrom", "?c"],
                    ["?r", "owl:onProperty", "?p"],
                    ["?s", "owl:allValuesFrom", "?d"],
                    ["?s", "owl:onProperty", "?p"],
                    ["?c", "rdfs:subClassOf", "?d"],
                ],
            ),
            Self::ScmAvf2 => (
                ["?s", "rdfs:subClassOf", "?r"],
                &[
                    ["?r", "owl:allValuesFrom", "?c"],
                    ["?r", "owl:onProperty", "?p"],
                    ["?s", "owl:allValuesFrom", "?c"],
                    ["?s", "owl:onProperty", "?q"],
                    ["?p", "rdfs:subPropertyOf", "?q"],
                ],
            ),
            Self::ScmOpSelfSubproperty => (
                ["?p", "rdfs:subPropertyOf", "?p"],
                &[["?p", "rdf:type", "owl:ObjectProperty"]],
            ),
            Self::ScmOpSelfEquivalent => (
                ["?p", "owl:equivalentProperty", "?p"],
                &[["?p", "rdf:type", "owl:ObjectProperty"]],
            ),
            Self::ScmDpSelfSubproperty => (
                ["?p", "rdfs:subPropertyOf", "?p"],
                &[["?p", "rdf:type", "owl:DatatypeProperty"]],
            ),
            Self::ScmDpSelfEquivalent => (
                ["?p", "owl:equivalentProperty", "?p"],
                &[["?p", "rdf:type", "owl:DatatypeProperty"]],
            ),
            _ => panic!("core clauses are constructed in RuleId::definition"),
        };
        Pattern(head).rule(body)
    }
}
