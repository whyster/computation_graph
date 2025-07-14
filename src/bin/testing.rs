use std::io;
use recursion::Collapsible;
use computation_graph::graph::node_frame::NodeFrame;
use computation_graph::graph::boxed_nodes::BoxedNode;
use computation_graph::graph::node_frame::BoolOp;
use computation_graph::graph::node_transformer::NodeTransformer;
use computation_graph::operations::BooleanOperation;

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

struct Ugh {}
impl NodeTransformer for Ugh {
    fn transform_boolean_operation(&mut self, parameter: BoolOp<Vec<Option<BoxedNode>>>) -> Option<BoxedNode> {
        let result = match &parameter.operator {
            BooleanOperation::And => {
                let mut state = true;
                for v in &parameter.operands {
                    let v = if let Some(a) = v { a } else {continue};
                    if let NodeFrame::BooleanConstant(b) = v.data {
                        state &= b;
                    } else {
                        return None;
                    }
                }
                Some(state)
            }
            BooleanOperation::Or => {
                let mut state = false;
                for v in &parameter.operands {
                    let v = if let Some(a) = v { a } else {continue};
                    if let NodeFrame::BooleanConstant(b) = v.data {
                        state |= b;
                    } else {
                        return None;
                    }
                }
                Some(state)
            }
        };
        
        if let Some(state) = result {
            Some ( bool_const!(state) )
        } else {
            Some( BoxedNode {
                data: NodeFrame::BoolOp( BoolOp {
                    operator: parameter.operator,
                    operands: parameter.operands.into_iter().filter_map(|n| n.map(Box::new)).collect(),
                })
            })
        }
    }
}

fn main() {
    let macro_graph: BoxedNode = bool_op!(BooleanOperation::And,
        bool_const!(true),
        bool_op!(BooleanOperation::Or,
            bool_const!(false),
            bool_const!(false)
        )
    );
    
    println!("Hello, world!");
    let mut ug = Ugh {};
    let result = ug.transform_node(macro_graph);
    result.unwrap();
    
}