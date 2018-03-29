
use super::parser::ast::*;

use std::collections::HashMap;
use std::io;
use std::ops::Range;
use std::error::Error;

mod functions;
mod value;

use self::value::MplValue;

// pub fn static_analysis(ast: Ast) -> Result<(), String> {
//     // TODO
// }

/// Evaluate the AST.
pub fn evaluate(ast: Ast) -> Result<(), String> {
    let mut interpreter = Interpreter { names: HashMap::new() };
    for stmt in ast.statements {
        match interpreter.evaluate_statement(stmt) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

struct Interpreter {
    names: HashMap<String, Vec<MplValue>>,
}

impl Interpreter {
    fn evaluate_statement(&mut self, statement: Statement) -> Result<(), String> {
        let status = match statement {
            Statement::Declaration { identifier, mpl_type, value }
                => self.evaluate_declaration(identifier, &mpl_type, value),
            Statement::Assignment { identifier, expression }
                => self.evaluate_assign(identifier, expression),
            Statement::For { identifier, begin, end, statements } => {
                let begin = self.expect_int_expr(begin)?;
                let end = self.expect_int_expr(end)? + 1;
                self.evaluate_for(identifier, begin .. end, statements)
            },
            Statement::Read(identifier) => self.evaluate_read(identifier),
            Statement::Print(expr) => self.evaluate_print(expr),
            Statement::Assert(expr) => self.evaluate_assert(expr),
            Statement::Empty => Ok(()),
        };
        match status {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn evaluate_for(&mut self, identifier: String, range: Range<i32>, statements: Vec<Statement>) -> Result<(), String> {
        for i in range {
            match self.names.get_mut(&identifier) {
                Some(stack) => {
                    stack.pop();
                    stack.push(MplValue::Int(i));
                },
                None => return Err("For loop identifier not initialized".to_string()),
            }
            for stmt in statements.iter() {
                self.evaluate_statement(stmt.clone())?;
            }
        }
        Ok(())
    }

    fn evaluate_declaration(&mut self, identifier: String, mpl_type: &MplType, value: Option<Expression>) -> Result<(), String> {
        let init = match value {
            Some(expr) => self.evaluate_expression(expr)?,
            None => MplValue::default(mpl_type), // initialize to default
        };
        if !init.is(mpl_type) {
            return Err(format!("Type {} does not match value {}", mpl_type, init));
        }
        self.names.insert(identifier, vec![init]);
        Ok(())
    }

    fn evaluate_assign(&mut self, identifier: String, val_expr: Expression) -> Result<(), String> {
        let value = self.evaluate_expression(val_expr)?;
        match self.names.get_mut(&identifier) {
            Some(stack) => { stack.pop(); stack.push(value); Ok(()) },
            None => Err("Identifier {} used before declaration".to_string()),
        }
    }

    fn evaluate_read(&mut self, identifier: String) -> Result<(), String> {
        let mut line = String::new();
        if let Err(e) = io::stdin().read_line(&mut line) {
            return Err(format!("IO error: {}", e));
        };
        let input = line.trim().to_string();
        let value = match self.get_type(&identifier)? {
            MplType::Int => self.parse_int(input)?,
            MplType::String => MplValue::String(input),
            MplType::Bool => self.parse_bool(input)?,
        };

        match self.names.get_mut(&identifier) {
            Some(stack) => { stack.pop(); stack.push(value); Ok(()) },
            None => Err("Identifier {} used before declaration".to_string()),
        }
    }

    fn evaluate_print(&self, print: Expression) -> Result<(), String> {
        self.evaluate_expression(print)
            .and_then(|value| { println!("{}", value); Ok(()) })
    }

    fn evaluate_assert(&self, assertion: Expression) -> Result<(), String> {
        self.evaluate_expression(assertion).and_then(|value| {
            match value {
                MplValue::Bool(true) => Ok(()),
                MplValue::Bool(false) => Err("Assertion failed".to_string()),
                value => Err(format!("Assert expected boolean argument, got {}", value.mpl_type())),
            }
        })
    }

    fn expect_int_expr(&self, expr: Expression) -> Result<i32, String> {
        self.evaluate_expression(expr).and_then(|value| match value {
            MplValue::Int(i) => Ok(i),
            MplValue::String(_) => Err("Expected int here, got string".to_string()),
            MplValue::Bool(_) => Err("Expected int here, got bool".to_string()),
        })
    }

    fn evaluate_expression(&self, expr: Expression) -> Result<MplValue, String> {
        match expr {
            Expression::Simple(opnd) => self.evaluate_operand(opnd),
            Expression::Binary { operator, left, right} => {
                let func = functions::MplFunction { sign: operator };
                func.call(self.evaluate_operand(left)?, self.evaluate_operand(right)?)
            },
            Expression::Unary { operator, operand } => {
                let func = functions::MplFunction { sign: operator };
                func.call_unary(self.evaluate_operand(operand)?)
            },
        }
    }

    fn evaluate_operand(&self, operand: Operand) -> Result<MplValue, String> {
        match operand {
            Operand::Int(i) => Ok(MplValue::Int(i)),
            Operand::String(s) => Ok(MplValue::String(s)),
            Operand::Identifier(id) => {
                match self.names.get(&id) {
                    Some(stack) => Ok(stack.last().unwrap().clone()), // last() == None should be impossible
                    None => Err(format!("Identifier {} used before assignment", id))
                }
            },
            Operand::Expr(expr) => self.evaluate_expression(*expr),
        }
    }

    fn parse_int(&self, input: String) -> Result<MplValue, String> {
        match input.parse() {
            Ok(i) => Ok(MplValue::Int(i)),
            Err(e) => Err(e.description().to_string()),
        }
    }

    fn parse_bool(&self, input: String) -> Result<MplValue, String> {
        match input.parse() {
            Ok(b) => Ok(MplValue::Bool(b)),
            Err(e) => Err(e.description().to_string()),
        }
    }

    fn get_type(&self, identifier: &String) -> Result<MplType, String> {
        match self.names.get(identifier) {
            Some(stack) => match stack.last().unwrap() { // last() == None should be impossible
                &MplValue::Int(_) => Ok(MplType::Int),
                &MplValue::String(_) => Ok(MplType::String),
                &MplValue::Bool(_) => Ok(MplType::Bool),
            },
            None => Err(format!("Identifier {} has not been declared", identifier)),
        }
    }
}
