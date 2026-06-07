use codesift_core::{Error, Result};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StructuralQuery {
    pub raw: String,
    pub symbol_name: Option<String>,
    pub kind: Option<String>,
    pub path: Option<String>,
    pub refs_to: Option<String>,
    pub callers_of: Option<String>,
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
        if let Some(value) = token.strip_prefix("symbol:name=") {
            query.symbol_name = Some(value.to_string());
        } else if let Some(value) = token.strip_prefix("kind=") {
            query.kind = Some(value.to_string());
        } else if let Some(value) = token.strip_prefix("path:") {
            query.path = Some(value.to_string());
        } else if let Some(value) = token.strip_prefix("refs:to=") {
            query.refs_to = Some(value.to_string());
        } else if let Some(value) = token.strip_prefix("callers:of=") {
            query.callers_of = Some(value.to_string());
        } else {
            return Err(Error::message(format!("unknown filter '{token}'")));
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
}
