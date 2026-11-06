#![doc = include_str!("../README.md")]

mod list;
mod literal;
mod number;
mod owl_numeric;
mod owl_numeric_membership;
mod owl_numeric_rules;
mod owl_numeric_violation;
mod owlrl;
mod preset;
mod rule;

pub use list::{ListRuleId, RDF_LIST_MEMBER_RELATION};
pub use literal::{LITERAL_VALUE_RELATION, LiteralRuleId};
pub use number::{ExactNumber, ExactNumberError};
pub use owl_numeric::{OwlFiniteNumericValue, OwlNumericDatatype, OwlNumericValue};
pub use owl_numeric_membership::{
    OWL_NUMERIC_MEMBER_RELATION, OWL_NUMERIC_NONMEMBER_RELATION,
    OWL_NUMERIC_TYPE_VIOLATION_RELATION, OwlNumericMembershipRuleId,
};
pub use owl_numeric_rules::{OWL_NUMERIC_VALUE_RELATION, OwlNumericRuleId};
pub use owl_numeric_violation::OwlNumericTypeViolation;
pub use owlrl::Owlrl762RuleId;
pub use preset::{Exclusion, Preset, PresetParseError};
pub use rule::{RuleId, TRIPLE_RELATION};
