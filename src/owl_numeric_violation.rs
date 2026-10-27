//! Checked portable diagnostic for an asserted incompatible numeric datatype.

use crate::{OwlNumericDatatype, OwlNumericValue};

/// An asserted numeric type contradicted by its interpreted value.
///
/// `L` preserves the caller's exact literal identity (owned or borrowed).
/// Construction and deserialization reject a value that belongs to the asserted
/// datatype. This checks the value-space mismatch, not the lexical mapping or
/// the existence of an assertion in a graph; those remain caller obligations.
/// Real/rational checks are general OWL extensions, outside an OWL-RL selection.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "ViolationWire<L>")]
pub struct OwlNumericTypeViolation<L> {
    literal: L,
    value: OwlNumericValue,
    datatype: OwlNumericDatatype,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ViolationWire<L> {
    literal: L,
    value: OwlNumericValue,
    datatype: OwlNumericDatatype,
}

impl<L> OwlNumericTypeViolation<L> {
    /// Check one observed rdf:type assertion under the selected numeric domain.
    ///
    /// # Errors
    /// Returns a typed violation if the interpreted value does not belong to
    /// the asserted datatype. A successful check says nothing about lexical
    /// validity, other assertions or whole-graph consistency.
    pub fn check_assertion(
        literal: L,
        value: OwlNumericValue,
        datatype: OwlNumericDatatype,
    ) -> Result<(), Self> {
        if value.is_member_of(datatype) {
            return Ok(());
        }
        Err(Self {
            literal,
            value,
            datatype,
        })
    }

    /// The exact caller-owned identity of the asserted literal.
    pub fn literal(&self) -> &L {
        &self.literal
    }

    /// The interpreted numeric value witnessing nonmembership.
    pub fn value(&self) -> &OwlNumericValue {
        &self.value
    }

    /// The incompatible asserted datatype.
    pub fn datatype(&self) -> OwlNumericDatatype {
        self.datatype
    }
}

impl<L> TryFrom<ViolationWire<L>> for OwlNumericTypeViolation<L> {
    type Error = &'static str;

    fn try_from(wire: ViolationWire<L>) -> Result<Self, Self::Error> {
        match Self::check_assertion(wire.literal, wire.value, wire.datatype) {
            Err(violation) => Ok(violation),
            Ok(()) => Err("numeric type violation requires value-space nonmembership"),
        }
    }
}
