use recursion::{Expandable, MappableFrame, PartiallyApplied};
use crate::graph::node_frame::{BitVec, FunctionParameter, NodeFrame, Numeric};
use crate::graph::structure_key::StructuralIdentifier::Group;
use crate::operations::{BinaryOperation, BooleanOperation, ComparisonOperation, UnaryOperation};

#[derive(Debug, Clone, PartialEq)]
pub enum StructuralIdentifier {
    FunctionParameter(FunctionParameter),
    NumericConstant(Numeric),
    StringConstant(String),
    BooleanConstant(bool),
    BitVec(BitVec),
    BinOp(BinaryOperation),
    UnaryOp(UnaryOperation),
    BoolOp(BooleanOperation),
    Compare(Vec<ComparisonOperation>),
    If,
    Group(Vec<StructuralIdentifier>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructureKey {
    pub contents: Vec<StructuralIdentifier>
}

impl Expandable for StructureKey {
    type FrameToken = NodeFrame<PartiallyApplied>;

    fn from_frame(val: <Self::FrameToken as MappableFrame>::Frame<Self>) -> Self {
        let mut contents: Vec<StructuralIdentifier> = Vec::new();
        match val {
            NodeFrame::FunctionParameter(p) => {
                contents.push(StructuralIdentifier::FunctionParameter(p));
            }
            NodeFrame::NumericConstant(n) => {
                contents.push(StructuralIdentifier::NumericConstant(n));
            }
            NodeFrame::StringConstant(s) => {
                contents.push(StructuralIdentifier::StringConstant(s));
            }
            NodeFrame::BooleanConstant(b) => {
                contents.push(StructuralIdentifier::BooleanConstant(b));
            }
            NodeFrame::BitVec(a) => {
                contents.push(StructuralIdentifier::BitVec(a));
            }
            NodeFrame::BinOp(b) => {
                contents.push(StructuralIdentifier::BinOp(b.operation)); 
                contents.push(Group(vec![Group(b.left.contents), Group(b.right.contents)]));
            }
            NodeFrame::UnaryOp(c) => {
                contents.push(StructuralIdentifier::UnaryOp(c.operation));
                contents.push(Group(c.operand.contents));
            }
            NodeFrame::BoolOp(d) => {
                contents.push(StructuralIdentifier::BoolOp(d.operator));
                contents.push(Group(d.operands.into_iter().map(|key| Group(key.contents)).collect()));
            }
            NodeFrame::Compare(cmp) => {
                contents.push(StructuralIdentifier::Compare(cmp.operations));
                contents.push(Group(vec![Group(cmp.left.contents), Group(cmp.comparators.into_iter().map(|key| Group(key.contents)).collect())]));
            }
            NodeFrame::If(if_node) => {
                contents.push(StructuralIdentifier::If);
                contents.push(Group(vec![Group(if_node.condition.contents), Group(if_node.success.contents), Group(if_node.failure.contents)]));
            }
        }
        Self {
            contents,
        }
    }
}