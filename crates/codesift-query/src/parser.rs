use codesift_core::{Error, Result};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StructuralQuery {
    pub raw: String,
    pub symbol_name: Option<String>,
    pub kind: Option<String>,
    pub path: Option<String>,
    pub lang: Option<String>,
    pub refs_to: Option<String>,
    pub callers_of: Option<String>,
    pub depth: Option<u32>,
}

pub fn parse_query(input: &str) -> Result<StructuralQuery> {
    let raw = input.trim().to_string();
    if raw.is_empty() {
        return Err(Error::message("empty query"));
    }

    let mut query = StructuralQuery {
        raw: raw.clone(),
        ..Default::default()
    };

    for token in raw.split_whitespace() {
        let Some((key, value)) = token.split_once('=') else {
            return Err(Error::message(format!(
                "expected key=value filter, got '{token}'"
            )));
        };
        if value.is_empty() {
            return Err(Error::message(format!("empty value for filter '{key}'")));
        }

        match key {
            "symbol:name" => query.symbol_name = Some(value.to_string()),
            "kind" => query.kind = Some(value.to_string()),
            "path" => query.path = Some(value.to_string()),
            "lang" => query.lang = Some(value.to_string()),
            "refs:to" => query.refs_to = Some(value.to_string()),
            "callers:of" => query.callers_of = Some(value.to_string()),
            "depth" => {
                let depth: u32 = value
                    .parse()
                    .map_err(|_| Error::message(format!("invalid depth '{value}'")))?;
                query.depth = Some(depth);
            }
            _ => return Err(Error::message(format!("unknown filter '{key}'"))),
        }
    }

    Ok(query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiple_filters() {
        let q = parse_query("symbol:name=main kind=function").unwrap();
        assert_eq!(q.symbol_name.as_deref(), Some("main"));
        assert_eq!(q.kind.as_deref(), Some("function"));
    }

    #[test]
    fn parses_path_and_refs_with_colons_in_value() {
        let q = parse_query(
            "path=crates/**/parser.rs refs:to=sym://1/src/parser.rs#function:parse_query@316:1401",
        )
        .unwrap();
        assert_eq!(q.path.as_deref(), Some("crates/**/parser.rs"));
        assert_eq!(
            q.refs_to.as_deref(),
            Some("sym://1/src/parser.rs#function:parse_query@316:1401")
        );
    }

    #[test]
    fn parses_callers_with_depth() {
        let q = parse_query("callers:of=sym://1/x#function:f@0:1 depth=3").unwrap();
        assert_eq!(q.depth, Some(3));
    }

    #[test]
    fn rejects_colon_only_path_syntax() {
        assert!(parse_query("path:src/lib.rs").is_err());
    }
}
