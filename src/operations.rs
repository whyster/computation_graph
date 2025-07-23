use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Shl, Shr, Sub};
use std::process::Output;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BinaryOperation { 
    Add,
    Subtract,
    Divide,
    Multiply,
    BitwiseAnd,
    BitwiseLeftShift,
    BitwiseRightShift,
    BitwiseOr,
    BitwiseXor,
}

impl BinaryOperation {
    pub fn perform<L, R, O>(&self, left: L, right: R) -> O
    where L: Add<R, Output = O>
           + Sub<R, Output = O>
           + Div<R, Output = O>
           + Mul<R, Output = O>
           + BitAnd<R, Output = O>
           + BitOr<R, Output = O>
           + BitXor<R, Output = O>
           + Shl<R, Output = O>
           + Shr<R, Output = O> {
        match self {
            BinaryOperation::Add => left + right,
            BinaryOperation::Subtract => left - right,
            BinaryOperation::Divide => left / right,
            BinaryOperation::Multiply => left * right,
            BinaryOperation::BitwiseAnd => left & right,
            BinaryOperation::BitwiseLeftShift => left << right,
            BinaryOperation::BitwiseRightShift => left >> right,
            BinaryOperation::BitwiseOr => left | right,
            BinaryOperation::BitwiseXor => left ^ right,
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BooleanOperation {
    And,
    Or
}

impl BooleanOperation {
    pub fn perform(&self, left: bool, right: bool) -> bool
    {
        match self {
            BooleanOperation::And => left && right,
            BooleanOperation::Or => left || right,
        }
    }

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnaryOperation {
    Not,
    Invert,
    UnaryMinus,
}

impl UnaryOperation {
    pub fn perform<T, R>(&self, operand: T) -> R
    where T: Not<Output = R>
           + Neg<Output = R>{
        match self {
            UnaryOperation::Not => !operand,
            UnaryOperation::Invert => !operand,
            UnaryOperation::UnaryMinus => -operand,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ComparisonOperation {
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    In,
    Is,
    IsNot,
    LessThan,
    LessThanOrEqual,
    NotEqual,
    NotIn,
}