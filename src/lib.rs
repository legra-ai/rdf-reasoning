#![doc = include_str!("../README.md")]

mod owlrl;
mod preset;
mod rule;

pub use owlrl::Owlrl762RuleId;
pub use preset::{Exclusion, Preset, PresetParseError};
pub use rule::{RuleId, TRIPLE_RELATION};
