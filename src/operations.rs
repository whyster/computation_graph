
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BooleanOperation {
    And,
    Or
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnaryOperation {
    Not,
    Invert,
    UnaryPlus,
    UnaryMinus,
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