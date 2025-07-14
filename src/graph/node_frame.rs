use recursion::{MappableFrame, PartiallyApplied};
use crate::operations;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionParameter {
    pub identifier: String,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BitVec {
    pub length: usize,
    pub bit_string: String,
}
#[derive(Debug, Clone)]
pub struct BinOp<T> {
    pub operation: operations::BinaryOperation,
    pub left: T,
    pub right: T,
}
#[derive(Debug, Clone)]
pub struct UnaryOp<T> {
    pub operation: operations::UnaryOperation,
    pub operand: T,
}
#[derive(Debug, Clone)]
pub struct BoolOp<U> {
    pub operator: operations::BooleanOperation,
    pub operands: U
}
#[derive(Debug, Clone)]
pub struct Compare<T, U> {
    pub left: T,
    pub operations: Vec<operations::ComparisonOperation>,
    pub comparators: U
}
#[derive(Debug, Clone)]
pub struct If<T> {
    pub condition: T,
    pub success: T,
    pub failure: T,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Numeric {
    Double(f64),
    Int(i32),
}
#[derive(Debug, Clone)]
pub enum NodeFrame<T> {
    FunctionParameter(FunctionParameter),
    NumericConstant(Numeric),
    StringConstant(String),
    BooleanConstant(bool),
    BitVec(BitVec),
    BinOp(BinOp<T>),
    UnaryOp(UnaryOp<T>),
    BoolOp(BoolOp<Vec<T>>),
    Compare(Compare<T, Vec<T>>),
    If(If<T>),
}


// PartiallyApplied can be replaced with any type parameter T
// But the way this trait is used (thanks to Frame<X>) if you use an arbitrary type parameter
// Then rust will generate separate implementations for every type, when that is unnecessary thanks
// to the structure of Frame<X>
//
// I'm sure there's a reason behind why the developer of the recursion library chose this approach
// over just making map_frame generic over NodeFrame<T> but whatever
impl MappableFrame for NodeFrame<PartiallyApplied> {
    type Frame<X> = NodeFrame<X>;

    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            NodeFrame::FunctionParameter(param) => NodeFrame::FunctionParameter(param),
            NodeFrame::NumericConstant(numeric) => NodeFrame::NumericConstant(numeric),
            NodeFrame::StringConstant(string) => NodeFrame::StringConstant(string),
            NodeFrame::BooleanConstant(boolean) => NodeFrame::BooleanConstant(boolean),
            NodeFrame::BitVec(bitvec) => NodeFrame::BitVec(bitvec),
            NodeFrame::BinOp(binop) => {
                NodeFrame::BinOp(BinOp {
                    operation: binop.operation,
                    left: f(binop.left),
                    right: f(binop.right),
                })
            }
            NodeFrame::UnaryOp(unaryop) => {
                NodeFrame::UnaryOp(UnaryOp {
                    operation: unaryop.operation,
                    operand: f(unaryop.operand),
                })
            }
            NodeFrame::BoolOp(boolop) => {
                NodeFrame::BoolOp(BoolOp {
                    operator: boolop.operator,
                    operands: boolop.operands.into_iter().map(f).collect(),
                })
            }
            NodeFrame::Compare(compare) => {
                NodeFrame::Compare(Compare {
                    left: f(compare.left),
                    operations: compare.operations,
                    comparators: compare.comparators.into_iter().map(f).collect(),
                })
            }
            NodeFrame::If(if_node) => {
                NodeFrame::If(If {
                    condition: f(if_node.condition),
                    success: f(if_node.success),
                    failure: f(if_node.failure),
                })
            }
        }
    }
}

impl<T> NodeFrame<T> {
    pub fn map<A, F: FnMut(T) -> A>(self, mut f: F) -> NodeFrame<A> {
        match self {
            NodeFrame::FunctionParameter(param) => NodeFrame::FunctionParameter(param),
            NodeFrame::NumericConstant(numeric) => NodeFrame::NumericConstant(numeric),
            NodeFrame::StringConstant(string) => NodeFrame::StringConstant(string),
            NodeFrame::BooleanConstant(boolean) => NodeFrame::BooleanConstant(boolean),
            NodeFrame::BitVec(bitvec) => NodeFrame::BitVec(bitvec),
            NodeFrame::BinOp(binop) => {
                NodeFrame::BinOp(BinOp {
                    operation: binop.operation,
                    left: f(binop.left),
                    right: f(binop.right),
                })
            }
            NodeFrame::UnaryOp(unaryop) => {
                NodeFrame::UnaryOp(UnaryOp {
                    operation: unaryop.operation,
                    operand: f(unaryop.operand),
                })
            }
            NodeFrame::BoolOp(boolop) => {
                NodeFrame::BoolOp(BoolOp {
                    operator: boolop.operator,
                    operands: boolop.operands.into_iter().map(f).collect(),
                })
            }
            NodeFrame::Compare(compare) => {
                NodeFrame::Compare(Compare {
                    left: f(compare.left),
                    operations: compare.operations,
                    comparators: compare.comparators.into_iter().map(f).collect(),
                })
            }
            NodeFrame::If(if_node) => {
                NodeFrame::If(If {
                    condition: f(if_node.condition),
                    success: f(if_node.success),
                    failure: f(if_node.failure),
                })
            }
        }
    }

}
