use std::ops::Not;
use crate::computing::Computable;
use crate::graph::boxed_nodes::BoxedNode;
use crate::graph::node_frame::{BinOp, BoolOp, Compare, NodeFrame, Numeric, UnaryOp};
use crate::graph::node_transformer::NodeTransformer;
use crate::operations::{BinaryOperation, BooleanOperation, UnaryOperation};

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

        let filtered_operands: Vec<Box<BoxedNode>> = parameter.operands.into_iter().filter_map(|x| Some(Box::new(x?))).collect();

        if filtered_operands.is_empty() {
            return None;
        }

        let mut value: bool = match parameter.operator {
            BooleanOperation::And => true,
            BooleanOperation::Or => false
        };

        for operand in &filtered_operands {
            // Can only simplify boolean operations when ALL operands are boolean constants
            if let NodeFrame::BooleanConstant(operand_value) = operand.data {
                value = parameter.operator.perform(value, operand_value);
            } else {
                return Some( BoxedNode {
                    data: NodeFrame::BoolOp(BoolOp {
                        operator: parameter.operator,
                        operands: filtered_operands // Currently returning operands with all None entries removed
                    })
                })
            }
        }
        Some( BoxedNode {
            data: NodeFrame::BooleanConstant(value)
        })
    }

    fn transform_comparison(&mut self, parameter: Compare<Option<BoxedNode>, Vec<Option<BoxedNode>>>) -> Option<BoxedNode> {
        let left = parameter.left?;
        let filtered_comparators: Vec<Box<BoxedNode>> = parameter.comparators.into_iter().filter_map(|x| Some(Box::new(x?))).collect();

        // There should be at least 1 element in both comparators and operations
        // The number of operations should also not exceed the number of comparators
        if parameter.operations.len() > filtered_comparators.len() {
            return None;
        }
        if filtered_comparators.is_empty() || parameter.operations.is_empty() {
            return None;
        }

        let mut expression_result: bool = parameter.operations[0]






        None
    }
}