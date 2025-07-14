use recursion::CollapsibleExt;
use crate::graph::boxed_nodes::BoxedNode;
use crate::graph::node_frame::{BinOp, BitVec, BoolOp, Compare, FunctionParameter, If, NodeFrame, Numeric, UnaryOp};

enum Value {
    Simplified,
    Unsimplified(Box<NodeFrame<Value>>),
}

pub trait NodeTransformer {
    fn transform_function_parameter(&mut self, parameter: FunctionParameter) -> Option<BoxedNode> {
        Some(BoxedNode {
            data: NodeFrame::FunctionParameter(parameter),
        })
    }

    fn transform_numeric_constant(&mut self, parameter: Numeric) -> Option<BoxedNode> {
        Some(BoxedNode {
            data: NodeFrame::NumericConstant(parameter),
        })
    }
    fn transform_string_constant(&mut self, parameter: String) -> Option<BoxedNode> {
        Some(BoxedNode {
            data: NodeFrame::StringConstant(parameter),
        })
    }

    fn transform_boolean_constant(&mut self, parameter: bool) -> Option<BoxedNode> {
        Some(BoxedNode {
            data: NodeFrame::BooleanConstant(parameter),
        })
    }

    fn transform_bitvec(&mut self, parameter: BitVec) -> Option<BoxedNode> {
        Some(BoxedNode {
            data: NodeFrame::BitVec(parameter),
        })
    }

    fn transform_binary_operation(&mut self, parameter: BinOp<Option<BoxedNode>>) -> Option<BoxedNode> {
        // Due to the default_visit collapse frames the tree should be visited in an order
        // Where the children were already visited before the parent
        let left_transformed_node = parameter.left?;
        let right_transformed_node = parameter.right?;
        // let left_transformed_node = self.transform_node(parameter.left?)?;
        // let right_transformed_node = self.transform_node(parameter.right?)?;
        Some(BoxedNode {
            data: NodeFrame::BinOp(BinOp {
                operation: parameter.operation,
                left: Box::new(left_transformed_node),
                right: Box::new(right_transformed_node),
            }),
        })

    }
    fn transform_unary_operation(&mut self, parameter: UnaryOp<Option<BoxedNode>>) -> Option<BoxedNode> {
        Some(BoxedNode {
            data: NodeFrame::UnaryOp(UnaryOp {
                operation: parameter.operation,
                operand: Box::new(parameter.operand?)
            }),
        })
    }

    fn transform_boolean_operation(&mut self, parameter: BoolOp<Vec<Option<BoxedNode>>>) -> Option<BoxedNode> {
        Some(BoxedNode {
            data: NodeFrame::BoolOp(BoolOp {
                operator: parameter.operator,
                operands: parameter.operands.into_iter()
                    .filter_map(|n| n.map(Box::new)).collect()
            })
        })
    }

    fn transform_comparison(&mut self, parameter: Compare<Option<BoxedNode>, Vec<Option<BoxedNode>>>) -> Option<BoxedNode> {
        Some(BoxedNode {
            data: NodeFrame::Compare(Compare {
                left: Box::new(parameter.left?),
                operations: parameter.operations,
                comparators: parameter.comparators.into_iter().filter_map(|n| n.map(Box::new)).collect(),
            })
        })
    }

    fn transform_if(&mut self, parameter: If<Option<BoxedNode>>) -> Option<BoxedNode> {
        Some(BoxedNode {
            data: NodeFrame::If(If {
                condition: Box::new(parameter.condition?),
                success: Box::new(parameter.success?),
                failure: Box::new(parameter.failure?),
            })
        })
    }

    fn transform_node(&mut self, node: BoxedNode) -> Option<BoxedNode> {
        self.default_visit(node)
    }
    fn default_visit(&mut self, node: BoxedNode) -> Option<BoxedNode> {
        node.collapse_frames(|x| {
            match x {
                NodeFrame::FunctionParameter(p) => self.transform_function_parameter(p),
                NodeFrame::NumericConstant(n) => self.transform_numeric_constant(n),
                NodeFrame::StringConstant(s) => self.transform_string_constant(s),
                NodeFrame::BooleanConstant(b) => self.transform_boolean_constant(b),
                NodeFrame::BitVec(bv) => self.transform_bitvec(bv),
                NodeFrame::BinOp(binop) => self.transform_binary_operation(binop),
                NodeFrame::UnaryOp(unaryop) => self.transform_unary_operation(unaryop),
                NodeFrame::BoolOp(boolop) => self.transform_boolean_operation(boolop),
                NodeFrame::Compare(compareop) => self.transform_comparison(compareop),
                NodeFrame::If(ifnode) => self.transform_if(ifnode),
            }
        })
    }
}

fn drop_non_vals(node: BoxedNode) {
    let x = node.collapse_frames(|x| {
        match x {
            NodeFrame::FunctionParameter(_) => Value::Simplified,
            NodeFrame::NumericConstant(_) => Value::Simplified,
            NodeFrame::StringConstant(_) => Value::Simplified,
            NodeFrame::BooleanConstant(_) => Value::Simplified,
            NodeFrame::BitVec(_) => Value::Simplified,
            NodeFrame::BinOp(binop) => {
                Value::Unsimplified(Box::new(NodeFrame::BinOp(binop)))
            }
            NodeFrame::UnaryOp(a) => {
                Value::Unsimplified(Box::new(NodeFrame::UnaryOp(a)))
            }
            NodeFrame::BoolOp(_) => {todo!()}
            NodeFrame::Compare(_) => {todo!()}
            NodeFrame::If(_) => {todo!()}
        }
    });
    match x {
        Value::Simplified => {}
        Value::Unsimplified(_) => {}
    }
}



#[cfg(test)]
mod transformer_tests {
    use recursion::{Collapsible, Expandable};
    use crate::operations::BooleanOperation;
    use super::*;

    //
    //  Macro syntax should probably look like this:
    //  BoolOp!(
    //      operator: BooleanOperation::And
    //      operands: vec![BoolConst!(true)]
    //  )
    //
    //
    macro_rules! bool_op {
        ($operator:expr, $($op:expr),*) => {
            BoxedNode {
                data: NodeFrame::BoolOp(BoolOp {
                    operator: $operator,
                    operands: vec![$(Box::new($op),)*]
                })
            }
        };
    }
    
    macro_rules! bool_const {
        ($val:expr) => {
            BoxedNode {
                data: NodeFrame::BooleanConstant($val),
            }
        };
    }
    
    struct DefaultImplementor {}
    impl NodeTransformer for DefaultImplementor {}
    
    #[test]
    fn default_implementation_does_not_mutate_structure() {
        let macro_graph: BoxedNode = bool_op!(BooleanOperation::And, 
            bool_const!(true),
            bool_op!(BooleanOperation::Or,
                bool_const!(false),
                bool_const!(true)
            )
        );
        let mut default_implementor = DefaultImplementor {};
        
        let result = default_implementor.transform_node(macro_graph);
        
        
        assert!(false)
        
        // let graph: BoxedNode = BoxedNode {
        //     data: NodeFrame::BoolOp(BoolOp {
        //         operator: BooleanOperation::And,
        //         operands: vec![Box::new(BoxedNode{ data: NodeFrame::BooleanConstant(true)})]
        //     }),
        // };
    }
}