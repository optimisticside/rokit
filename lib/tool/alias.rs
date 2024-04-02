use std::{fmt, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};
use thiserror::Error;

use super::{util::is_invalid_identifier, ToolId};

/**
    Error type representing the possible errors that can occur when parsing a `ToolAlias`.
*/
#[derive(Debug, Error)]
pub enum ToolAliasParseError {
    #[error("alias is empty")]
    Empty,
    #[error("alias '{0}' is invalid")]
    Invalid(String),
    #[error("alias '{0}' contains whitespace")]
    ContainsWhitespace(String),
}

/**
    A tool alias, which is a simple string identifier for a tool.

    Tool aliases are not case sensitive, and forced to lowercase when parsed.

    Used in:

    - Manifest keys, as a shorthand for a tool's author and name.
    - Executable names, as the main identifier.
*/
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, DeserializeFromStr, SerializeDisplay,
)]
pub struct ToolAlias {
    pub(super) name: String,
}

impl ToolAlias {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl FromStr for ToolAlias {
    type Err = ToolAliasParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ToolAliasParseError::Empty);
        }
        if is_invalid_identifier(s) {
            return Err(ToolAliasParseError::Invalid(s.to_string()));
        }
        if s.chars().any(char::is_whitespace) {
            return Err(ToolAliasParseError::ContainsWhitespace(s.to_string()));
        }
        if s.eq_ignore_ascii_case("rokit") {
            return Err(ToolAliasParseError::Invalid(s.to_string()));
        }
        Ok(Self {
            name: s.to_ascii_lowercase(),
        })
    }
}

impl fmt::Display for ToolAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)
    }
}

impl From<&ToolId> for ToolAlias {
    fn from(id: &ToolId) -> Self {
        Self {
            name: id.name().to_ascii_lowercase(),
        }
    }
}

impl From<ToolId> for ToolAlias {
    fn from(id: ToolId) -> Self {
        (&id).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_alias(name: &str) -> ToolAlias {
        ToolAlias {
            name: name.to_string(),
        }
    }

    #[test]
    fn parse_valid_basic() {
        // Basic strings should parse ok
        assert!("a".parse::<ToolAlias>().is_ok());
        assert!("tool".parse::<ToolAlias>().is_ok());
        assert!("tool-alias".parse::<ToolAlias>().is_ok());
        assert!("tool_alias".parse::<ToolAlias>().is_ok());
        // The parsed ToolName should match the input
        assert_eq!("a".parse::<ToolAlias>().unwrap(), new_alias("a"));
        assert_eq!("tool".parse::<ToolAlias>().unwrap(), new_alias("tool"));
        assert_eq!(
            "tool-alias".parse::<ToolAlias>().unwrap(),
            new_alias("tool-alias")
        );
        assert_eq!(
            "tool_alias".parse::<ToolAlias>().unwrap(),
            new_alias("tool_alias")
        );
    }

    #[test]
    fn parse_invalid_empty() {
        // Empty strings should not parse
        assert!("".parse::<ToolAlias>().is_err());
    }

    #[test]
    fn parse_invalid_whitespace() {
        // Strings containing spaces should not parse
        assert!(" tool".parse::<ToolAlias>().is_err());
        assert!("tool ".parse::<ToolAlias>().is_err());
        assert!("to ol".parse::<ToolAlias>().is_err());
        // Strings containing newlines or tabs should not parse
        assert!("\ntool".parse::<ToolAlias>().is_err());
        assert!("tool\n".parse::<ToolAlias>().is_err());
        assert!("to\nol".parse::<ToolAlias>().is_err());
        assert!("\ttool".parse::<ToolAlias>().is_err());
        assert!("tool\t".parse::<ToolAlias>().is_err());
        assert!("to\tol".parse::<ToolAlias>().is_err());
    }

    #[test]
    fn case_sensitivity() {
        // ToolAliases should be case-insensitive
        assert_eq!(
            "tool".parse::<ToolAlias>().unwrap(),
            "TOOL".parse().unwrap()
        );
        assert_eq!(
            "tool".parse::<ToolAlias>().unwrap(),
            "Tool".parse().unwrap()
        );
        assert_eq!(
            "tool".parse::<ToolAlias>().unwrap(),
            "tOoL".parse().unwrap()
        );
    }
}
