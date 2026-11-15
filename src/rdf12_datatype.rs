//! Mandatory default datatype semantics from RDF 1.2 Semantics CRD 2026-09-24.
use std::{fmt, str::FromStr};

/// One datatype in the mandatory default RDF 1.2 recognition map.
///
/// This selects the three default datatypes, not every datatype encountered in
/// input. Structural literal metadata must separately pass standard facet
/// validation. Unknown input datatypes remain opaque under the default map.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Rdf12Datatype {
    /// XML Schema string with the XML 1.1 Char lexical restriction.
    #[serde(rename = "http://www.w3.org/2001/XMLSchema#string")]
    String,
    /// A lexical string and a structurally valid language tag.
    #[serde(rename = "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString")]
    LangString,
    /// A lexical string, language tag and base direction.
    #[serde(rename = "http://www.w3.org/1999/02/22-rdf-syntax-ns#dirLangString")]
    DirLangString,
}

impl Rdf12Datatype {
    /// All three mandatory default datatypes; consumers must include all three.
    pub const ALL: &'static [Self] = &[Self::String, Self::LangString, Self::DirLangString];

    /// Exact datatype identity, also used by Serde and strict configuration parsing.
    #[must_use]
    pub const fn iri(self) -> &'static str {
        match self {
            Self::String => "http://www.w3.org/2001/XMLSchema#string",
            Self::LangString => "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString",
            Self::DirLangString => "http://www.w3.org/1999/02/22-rdf-syntax-ns#dirLangString",
        }
    }

    /// Recognize an encountered IRI in the fixed default map without coercion.
    ///
    /// `None` means an opaque encountered datatype. Requested recognition must
    /// instead use strict parsing, which rejects unsupported map extensions.
    #[must_use]
    pub fn recognize_iri(iri: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|datatype| datatype.iri() == iri)
    }

    /// Validate one UTF-8 lexical string or independently decoded string fragment.
    ///
    /// Language and directional strings have no ill-typed lexical payloads after
    /// structural validation. This does not validate facets, tags, directions or
    /// graph consistency. No value normalization or source-term replacement occurs.
    /// Streaming callers own UTF-8 decoding and accumulation of fragment offsets.
    /// # Errors
    /// Returns the first XML 1.1 Char violation for `xsd:string`, with the byte
    /// offset relative to the supplied string fragment.
    pub fn validate_lexical(self, lexical: &str) -> Result<(), Rdf12LexicalError> {
        if self != Self::String {
            return Ok(());
        }
        for (byte_offset, character) in lexical.char_indices() {
            if !Self::is_xml11_char(character) {
                return Err(Rdf12LexicalError {
                    byte_offset: u64::try_from(byte_offset)
                        .expect("UTF-8 fragment byte offsets must fit in u64"),
                    character,
                });
            }
        }
        Ok(())
    }

    fn is_xml11_char(character: char) -> bool {
        matches!(character, '\u{1}'..='\u{d7ff}' | '\u{e000}'..='\u{fffd}' | '\u{10000}'..='\u{10ffff}')
    }
}

/// A requested recognized datatype is outside the supported default map.
///
/// This is a configuration error, not a claim that an encountered opaque
/// literal is ill-typed. Callers retain the rejected configuration identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnsupportedRdf12Datatype;

impl fmt::Display for UnsupportedRdf12Datatype {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("unsupported requested RDF 1.2 datatype recognition")
    }
}

impl std::error::Error for UnsupportedRdf12Datatype {}

impl FromStr for Rdf12Datatype {
    type Err = UnsupportedRdf12Datatype;

    fn from_str(iri: &str) -> Result<Self, Self::Err> {
        Self::recognize_iri(iri).ok_or(UnsupportedRdf12Datatype)
    }
}

/// One invalid XML 1.1 character in an `xsd:string` lexical fragment.
///
/// The source literal identity and fragment's global offset remain caller-owned.
/// Checked deserialization rejects reports whose character is valid XML 1.1.
/// An offset alone cannot certify an external literal or graph assertion.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "LexicalErrorInput")]
pub struct Rdf12LexicalError {
    byte_offset: u64,
    character: char,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct LexicalErrorInput {
    byte_offset: u64,
    character: char,
}

impl Rdf12LexicalError {
    /// Byte offset of the first invalid character in the supplied UTF-8 fragment.
    #[must_use]
    pub const fn byte_offset(&self) -> u64 {
        self.byte_offset
    }

    /// The invalid Unicode scalar; UTF-8 decoding validity is checked by the caller.
    #[must_use]
    pub const fn character(&self) -> char {
        self.character
    }
}

impl TryFrom<LexicalErrorInput> for Rdf12LexicalError {
    type Error = &'static str;

    fn try_from(input: LexicalErrorInput) -> Result<Self, Self::Error> {
        if Rdf12Datatype::is_xml11_char(input.character) {
            return Err("RDF 1.2 lexical error requires an invalid XML 1.1 character");
        }
        Ok(Self {
            byte_offset: input.byte_offset,
            character: input.character,
        })
    }
}

impl fmt::Display for Rdf12LexicalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid XML 1.1 character U+{:04X} at lexical byte {}",
            self.character as u32, self.byte_offset
        )
    }
}

impl std::error::Error for Rdf12LexicalError {}
