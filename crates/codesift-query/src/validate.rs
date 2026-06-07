//! Validate and enrich structural query parsing.

use codesift_core::{Error, Result};

use crate::StructuralQuery;

/// Reject refs/callers targets that are not symbol IDs (avoids store serde errors).
pub fn validate_graph_targets(query: &StructuralQuery) -> Result<()> {
    if let Some(to_id) = &query.refs_to {
        validate_symbol_target(to_id, "refs:to")?;
    }
    if let Some(to_id) = &query.callers_of {
        validate_symbol_target(to_id, "callers:of")?;
    }
    Ok(())
}

fn validate_symbol_target(value: &str, filter: &str) -> Result<()> {
    if value.starts_with("sym://") || value.contains("/unresolved#") {
        return Ok(());
    }
    Err(Error::message(format!(
        "invalid {filter} value '{value}': expected sym://… symbol id (use refs --name for lookup by name)"
    )))
}

/// Human-oriented hint appended to parse errors when the raw query suggests a typo.
pub fn parse_query_with_hints(input: &str) -> Result<StructuralQuery> {
    parse_query_inner(input).map_err(|err| {
        let msg = err.to_string();
        if msg.contains("unknown filter") && input.contains("callers:") && !input.contains("callers:of=")
        {
            return Error::message(format!(
                "{msg}; did you mean callers:of=<sym_id> depth=<n>?"
            ));
        }
        if msg.contains("unknown filter") && input.contains("refs:") && !input.contains("refs:to=") {
            return Error::message(format!(
                "{msg}; did you mean refs:to=<sym_id> or use `codesift refs --name <name>`?"
            ));
        }
        err
    })
}

fn parse_query_inner(input: &str) -> Result<StructuralQuery> {
    use crate::parser::parse_query;
    let query = parse_query(input)?;
    validate_graph_targets(&query)?;
    Ok(query)
}

// Re-export raw parser for tests; CLI should prefer parse_query_with_hints.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_refs_to_name_shorthand() {
        let err = parse_query_with_hints("refs:to=name:parse_query").unwrap_err();
        assert!(err.to_string().contains("expected sym://"));
    }

    #[test]
    fn accepts_sym_id_refs_to() {
        let q = parse_query_with_hints(
            "refs:to=sym://1/crates/a.rs#function:parse_query@0:1",
        )
        .unwrap();
        assert!(q.refs_to.is_some());
    }

    #[test]
    fn hints_on_callers_typo() {
        let err = parse_query_with_hints("callers:to=sym://1/x#function:f@0:1").unwrap_err();
        assert!(err.to_string().contains("callers:of="));
    }
}
