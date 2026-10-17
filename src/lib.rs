#![doc = include_str!("../README.md")]

mod preset;
mod rule;

pub use preset::{Exclusion, Preset, PresetParseError};
pub use rule::{RuleId, TRIPLE_RELATION};
