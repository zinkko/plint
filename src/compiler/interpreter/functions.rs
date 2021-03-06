
use super::MplValue;

/// An MplFunction represents the function defined by the operator
pub struct MplFunction {
    pub sign: char,
}

impl MplFunction {
    /// Call a unary function. Perform type checking. If the operator is not a unary operator, return an error.
    pub fn call_unary(&self, operand: MplValue) -> Result<MplValue, String> {
        match self.sign {
            '!' => Ok(MplValue::Bool(!operand.to_bool()?)),
            // '-' => Ok(MplValue::Int(-operand.to_int()?)),
            _ => Err(format!("Operator {} is not a unary operator", self.sign)),
        }
    }

    /// Call a binary function. Performs type checking. If the operator is not a binary operator, return an error.
    pub fn call(&self, left: MplValue, right: MplValue) -> Result<MplValue, String>{
        match self.sign {
            '+' => plus(left, right),
            '-' => Ok(MplValue::Int(left.to_int()? - right.to_int()?)),
            '/' => Ok(MplValue::Int(left.to_int()? / right.to_int()?)),
            '*' => Ok(MplValue::Int(left.to_int()? * right.to_int()?)),
            '&' => Ok(MplValue::Bool(left.to_bool()? && right.to_bool()?)),
            '=' => Ok(MplValue::Bool(left == right)),
            '<' => compare(left, right),
            '!' => Err("! is a unary operator".to_string()),
            wrong => Err(format!("Unknown operator: {}", wrong)),
        }
    }

}

/// Helper functions for comparisons. Internal use only.
fn compare(left: MplValue, right: MplValue) -> Result<MplValue, String> {
    match left {
        MplValue::Int(i) => Ok(MplValue::Bool(i < right.to_int()?)),
        MplValue::String(s) => Ok(MplValue::Bool(s < right.to_string()?)),
        MplValue::Bool(b) => Ok(MplValue::Bool(b < right.to_bool()?)),
    }
}

/// Helper function for addition. Handles addition of both integers and strings.
fn plus(left: MplValue, right: MplValue) -> Result<MplValue, String> {
    match left {
        MplValue::Int(i) => Ok(MplValue::Int(i + right.to_int()?)),
        MplValue::String(s) => Ok(MplValue::String(format!("{}{}", s, right.to_string()?))),
        MplValue::Bool(_) => Err("Expected integer or string, got boolean".to_string()),
    }
}
