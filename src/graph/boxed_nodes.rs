use std::ops::Deref;
use recursion::{Collapsible, CollapsibleExt, Expandable, MappableFrame, PartiallyApplied};
use crate::graph::node_frame::{BinOp, UnaryOp, NodeFrame, BoolOp, Compare, If};
use crate::graph::structure_key::StructureKey;

#[derive(Debug, Clone)]
pub struct BoxedNode {
    pub data: NodeFrame<Box<BoxedNode>>,
}

impl BoxedNode {
    pub fn get_structure_key(self) -> StructureKey {
        let x = self.collapse_frames(|x| {
           StructureKey::from_frame(x)
        });
        x
    }
}


impl Expandable for BoxedNode {
    type FrameToken = NodeFrame<PartiallyApplied>;

    fn from_frame(val: <Self::FrameToken as MappableFrame>::Frame<Self>) -> Self {
        let boxed: NodeFrame<Box<BoxedNode>> = match val {
            NodeFrame::FunctionParameter(p) => NodeFrame::FunctionParameter(p),
            NodeFrame::NumericConstant(n) => NodeFrame::NumericConstant(n),
            NodeFrame::StringConstant(s) => NodeFrame::StringConstant(s),
            NodeFrame::BooleanConstant(b) => NodeFrame::BooleanConstant(b),
            NodeFrame::BitVec(bv) => NodeFrame::BitVec(bv),
            NodeFrame::BinOp(binop) => {
                let new_binop = BinOp {
                    operation: binop.operation,
                    left: Box::new(binop.left),
                    right: Box::new(binop.right),
                };
                NodeFrame::BinOp(new_binop)
            },
            NodeFrame::UnaryOp(unaryop) => {
                let new_unaryop = UnaryOp {
                    operation: unaryop.operation,
                    operand: Box::new(unaryop.operand),
                };
                NodeFrame::UnaryOp(new_unaryop)
            }
            NodeFrame::BoolOp(boolop) => {
                let new_boolop: BoolOp<Vec<Box<BoxedNode>>> = BoolOp {
                    operator: boolop.operator,
                    operands: boolop.operands.into_iter().map(Box::new).collect(),
                };
                NodeFrame::BoolOp(new_boolop)
            }
            NodeFrame::Compare(compare) => {
                let new_compare: Compare<Box<BoxedNode>, Vec<Box<BoxedNode>>> = Compare {
                    left: Box::new(compare.left),
                    operations: compare.operations,
                    comparators: compare.comparators.into_iter().map(Box::new).collect(),
                };
                NodeFrame::Compare(new_compare)
            }
            NodeFrame::If(if_node) => {
                let new_if = If {
                    condition: Box::new(if_node.condition),
                    success: Box::new(if_node.success),
                    failure: Box::new(if_node.failure),
                };
                NodeFrame::If(new_if)
            }
        };
        Self {
            data: boxed,
        }
    }
}

impl Collapsible for BoxedNode {
    type FrameToken = NodeFrame<PartiallyApplied>;

    fn into_frame(self) -> <Self::FrameToken as MappableFrame>::Frame<Self> {
        match self.data {
            NodeFrame::FunctionParameter(p) => NodeFrame::FunctionParameter(p),
            NodeFrame::NumericConstant(n) => NodeFrame::NumericConstant(n),
            NodeFrame::StringConstant(s) => NodeFrame::StringConstant(s),
            NodeFrame::BooleanConstant(b) => NodeFrame::BooleanConstant(b),
            NodeFrame::BitVec(bv) => NodeFrame::BitVec(bv),
            NodeFrame::BinOp(binop) => {
                let new_binop = BinOp {
                    operation: binop.operation,
                    left: *binop.left,
                    right: *binop.right,
                };
                NodeFrame::BinOp(new_binop)
            }
            NodeFrame::UnaryOp(unaryop) => {
                let new_unary = UnaryOp {
                    operation: unaryop.operation,
                    operand: *unaryop.operand,
                };
                NodeFrame::UnaryOp(new_unary)
            }
            NodeFrame::BoolOp(boolop) => {
                let new_bool = BoolOp {
                    operator: boolop.operator,
                    operands: boolop.operands.into_iter().map(|n| *n).collect(),
                };
                NodeFrame::BoolOp(new_bool)
            }
            NodeFrame::Compare(compare) => {
                let new_compare = Compare {
                    left: *compare.left,
                    operations: compare.operations,
                    comparators: compare.comparators.into_iter().map(|n| *n).collect(),
                };
                NodeFrame::Compare(new_compare)
            }
            NodeFrame::If(if_node) => {
                let new_if = If {
                    condition: *if_node.condition,
                    success: *if_node.success,
                    failure: *if_node.failure,
                };
                NodeFrame::If(new_if)
            }
        }
    }
}