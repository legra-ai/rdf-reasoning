//! Default datatype semantics from RDF 1.2 Semantics CRD 2026-09-24.
use rdf_reasoning::{Rdf12Datatype as D, Rdf12LexicalError};

const STRING: &str = "http://www.w3.org/2001/XMLSchema#string";
const LANG: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString";
const DIR: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#dirLangString";

#[test]
fn requested_recognition_is_strict_while_encountered_unknown_datatypes_are_opaque() {
    assert_eq!(D::ALL, &[D::String, D::LangString, D::DirLangString]);
    for (iri, datatype) in [
        (STRING, D::String),
        (LANG, D::LangString),
        (DIR, D::DirLangString),
    ] {
        assert_eq!(datatype.iri(), iri);
        assert_eq!(D::recognize_iri(iri), Some(datatype));
        assert_eq!(iri.parse::<D>(), Ok(datatype));
        assert_eq!(
            serde_json::to_string(&datatype).unwrap(),
            format!("\"{iri}\"")
        );
    }
    for unknown in [
        "urn:opaque",
        "xsd:string",
        "http://www.w3.org/2001/XMLSchema#String",
        "http://www.w3.org/2001/XMLSchema#integer",
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral",
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#JSON",
    ] {
        assert_eq!(D::recognize_iri(unknown), None);
        assert!(unknown.parse::<D>().is_err());
        assert!(serde_json::from_str::<D>(&format!("\"{unknown}\"")).is_err());
    }
}

#[test]
fn xsd_string_uses_xml11_not_xml10_character_boundaries() {
    for scalar in [
        '\u{1}',
        '\u{8}',
        '\t',
        '\n',
        '\u{b}',
        '\u{c}',
        '\r',
        '\u{e}',
        '\u{1f}',
        ' ',
        '\u{7f}',
        '\u{85}',
        '\u{9f}',
        '\u{d7ff}',
        '\u{e000}',
        '\u{fffd}',
        '\u{10000}',
        '\u{1fffe}',
        '\u{10ffff}',
    ] {
        let lexical = scalar.to_string();
        assert_eq!(
            D::String.validate_lexical(&lexical),
            Ok(()),
            "U+{:X}",
            scalar as u32
        );
    }
    assert_eq!(D::String.validate_lexical(""), Ok(()));
    for scalar in ['\0', '\u{fffe}', '\u{ffff}'] {
        let error = D::String
            .validate_lexical(&format!("é{scalar}tail"))
            .unwrap_err();
        assert_eq!(error.character(), scalar);
        assert_eq!(error.byte_offset(), 2);
    }
    let first = D::String.validate_lexical("a\0\u{ffff}").unwrap_err();
    assert_eq!(first.character(), '\0');
    assert_eq!(first.byte_offset(), 1);
}

#[test]
fn legal_language_and_directional_lexical_payloads_are_not_xml_restricted() {
    // Tags/direction/facet combinations are checked by the RDF structural model.
    for datatype in [D::LangString, D::DirLangString] {
        for lexical in ["", "text", "\0", "\u{fffe}", "\u{ffff}", "a\u{1}é"] {
            assert_eq!(datatype.validate_lexical(lexical), Ok(()));
        }
    }
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Nested {
    datatype: D,
    error: Option<Rdf12LexicalError>,
}

#[test]
fn checked_nested_json_and_binary_preserve_datatypes_and_real_lexical_errors() {
    for datatype in D::ALL {
        let value = Nested {
            datatype: *datatype,
            error: datatype.validate_lexical("é\0").err(),
        };
        assert_eq!(
            serde_json::from_str::<Nested>(&serde_json::to_string(&value).unwrap()).unwrap(),
            value
        );
        assert_eq!(
            postcard::from_bytes::<Nested>(&postcard::to_allocvec(&value).unwrap()).unwrap(),
            value
        );
    }
    #[derive(serde::Serialize)]
    struct RawError {
        byte_offset: u64,
        character: char,
    }
    for valid in ['a', '\u{1}', '\u{fffd}', '\u{10ffff}'] {
        let raw = RawError {
            byte_offset: 0,
            character: valid,
        };
        assert!(
            serde_json::from_str::<Rdf12LexicalError>(&serde_json::to_string(&raw).unwrap())
                .is_err()
        );
        assert!(
            postcard::from_bytes::<Rdf12LexicalError>(&postcard::to_allocvec(&raw).unwrap())
                .is_err()
        );
    }
    assert!(
        serde_json::from_str::<Rdf12LexicalError>(
            r#"{"byte_offset":0,"character":"\u0000","extra":true}"#
        )
        .is_err()
    );
}
