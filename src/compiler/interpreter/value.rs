use std::fmt;
use std::fmt::Display;

use super::super::parser::ast::MplType;

/// A value in the MiniPl language.
#[derive(Clone, Debug, PartialEq)]
pub enum MplValue {
    Int(i32),
    String(String),
    Bool(bool),
}

impl Display for MplValue {
    // Define fmt to make MplValues printable.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &MplValue::Int(i) => write!(f, "{}", i),
            &MplValue::String(ref s) => write!(f, "{}", s),
            &MplValue::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl MplValue {
    /// Check if the value of a certain type.
    pub fn is(&self, mpl_type: &MplType) -> bool {
        match (self, mpl_type) {
            (&MplValue::Int(_), &MplType::Int) => true,
            (&MplValue::String(_), &MplType::String) => true,
            (&MplValue::Bool(_), &MplType::Bool) => true,
            _ => false,
        }
    }

    /// Attempt to convert the value to a rust integer. Return Error if the type is incompatible.
    pub fn to_int(self) -> Result<i32, String> {
        match self {
            MplValue::Int(i) => Ok(i),
            MplValue::String(_) => Err("Expected integer, got string".to_string()),
            MplValue::Bool(_) => Err("Expected integer, got boolean".to_string()),
        }
    }
    /// Attempt to convert the value to a rust string. Return Error if the type is incompatible.
    pub fn to_string(self) -> Result<String, String> {
        match self {
            MplValue::Int(_) => Err("Expected string, got integer".to_string()),
            MplValue::String(s) => Ok(s),
            MplValue::Bool(_) => Err("Expected string, got boolean".to_string()),
        }
    }
    /// Attempt to convert the value to a rust boolean. Return Error if the type is incompatible.
    pub fn to_bool(self) -> Result<bool, String> {
        match self {
            MplValue::Int(_) => Err("Expected boolean, got integer".to_string()),
            MplValue::String(_) => Err("Expected boolean, got string".to_string()),
            MplValue::Bool(b) => Ok(b),
        }
    }

    /// Return the type of the value. This allows cleaner code than matching on the MplValue.
    pub fn mpl_type(&self) -> MplType {
        match self {
            &MplValue::Int(_) => MplType::Int,
            &MplValue::String(_) => MplType::String,
            &MplValue::Bool(_) => MplType::Bool,
        }
    }

    /// Return the default value for the corresponding type.
    /// # Defaults
    /// - 0 is the default for Int
    /// - "" is the default for String
    /// - false is the default for Bool
    pub fn default(mpl_type: &MplType) -> MplValue {
        match mpl_type {
            &MplType::Int => MplValue::Int(0),
            &MplType::String => MplValue::String("".to_string()),
            &MplType::Bool => MplValue::Bool(false),
        }
    }
}
