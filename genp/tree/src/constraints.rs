use crate::nodes::*;

pub fn get_nodes<'a>() -> Nodes<&'a str> {
    let intermediate = intermediates();
    let leafs = leafs();
    Nodes::<&str> {
        intermediate,
        leafs,
    }
}

pub fn intermediates<'a>() -> Vec<IntermediateNode<&'a str>> {
    vec![
        IntermediateNode {
            value: "Sequence",
            arity: 2,
            random_arity: true,
        },
//        IntermediateNode {
//            value: "F",
//            arity: 2,
//            random_arity: true,
//        },
//        IntermediateNode {
//            value: "Parallel",
//            arity: 2,
//            random_arity: true,
//        },
//        IntermediateNode {
//            value: "REPEAT num_cycles='-1'",
//            arity: 1,
//            random_arity: false,
//        },
    ]
}

pub fn leafs<'a>() -> Vec<&'a str> {
    vec!["Action ID='RETRIEVE_CAP'", "Action ID='GET_BASE'", "Action ID='INPUT_BASE'",
    "Action ID='MOUNT_CAP'", "Action ID='DELIVER'"]
}
pub fn get_delim<'a>() -> (&'a str, &'a str) {
    ("(", ")")
}
