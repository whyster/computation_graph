
#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum BooleanOperation {
    And,
    Or
}

#[derive(Copy, Clone)]
pub enum UnaryOperation {
    Not,
    Invert,
    UnaryPlus,
    UnaryMinus,
}

#[derive(Copy, Clone)]
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