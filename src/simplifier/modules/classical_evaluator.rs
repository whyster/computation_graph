use std::ops::Not;
use crate::computing::Computable;
use crate::graph::boxed_nodes::BoxedNode;
use crate::graph::node_frame::{BinOp, BoolOp, Compare, NodeFrame, Numeric, UnaryOp};
use crate::graph::node_transformer::NodeTransformer;
use crate::operations::{BinaryOperation, UnaryOperation};

struct ClassicalEvaluator {}

impl NodeTransformer for ClassicalEvaluator {
    fn transform_binary_operation(&mut self, parameter: BinOp<Option<BoxedNode>>) -> Option<BoxedNode> {
        let left = parameter.left?;
        let right = parameter.right?;
        match (&left.data, &right.data) {
            (NodeFrame::NumericConstant(Numeric::Int(left_n)), NodeFrame::NumericConstant(Numeric::Int(right_n))) => {
                let value = parameter.operation.perform(left_n, right_n);

                Some(BoxedNode {
                    data: NodeFrame::NumericConstant(Numeric::Int(value)),
                })
            },
            (_, _) => {
                Some(BoxedNode {
                    data: NodeFrame::BinOp(BinOp {
                        operation: parameter.operation,
                        left: Box::new(left),
                        right: Box::new(right),
                    })
                })
            }
        }
    }
    fn transform_unary_operation(&mut self, parameter: UnaryOp<Option<BoxedNode>>) -> Option<BoxedNode> {
        let operand = parameter.operand?;
        match &operand.data {
            NodeFrame::NumericConstant(Numeric::Int(n)) => {
                let value = Numeric::Int(parameter.operation.perform(n));
                Some( BoxedNode {
                    data: NodeFrame::NumericConstant(value),
                })
            }
            NodeFrame::BooleanConstant(b) => {
                assert_ne!(parameter.operation, UnaryOperation::UnaryMinus,
                           "Unary minus applied to a boolean is non-sensical");

                let value = b.not();
                Some( BoxedNode {
                    data: NodeFrame::BooleanConstant(value),
                })
            }
            _ => {
                Some( BoxedNode {
                    data: NodeFrame::UnaryOp(UnaryOp {
                        operation: parameter.operation,
                        operand: Box::new(operand),
                    })
                })
            }
        }
    }

    fn transform_boolean_operation(&mut self, parameter: BoolOp<Vec<Option<BoxedNode>>>) -> Option<BoxedNode> {
        todo!()
    }

    fn transform_comparison(&mut self, parameter: Compare<Option<BoxedNode>, Vec<Option<BoxedNode>>>) -> Option<BoxedNode> {
        todo!()
    }
}