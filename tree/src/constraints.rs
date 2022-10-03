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
            value: "seq",
            arity: 2,
            random_arity: true,
        },
        IntermediateNode {
            value: "fall",
            arity: 2,
            random_arity: true,
        },
        IntermediateNode {
            value: "par",
            arity: 2,
            random_arity: true,
        },
        IntermediateNode {
            value: "pol",
            arity: 1,
            random_arity: false,
        },
    ]
}

pub fn leafs<'a>() -> Vec<&'a str> {
    vec!["pickup", "place", "move", "ready"]
}
pub fn get_delim<'a>() -> (&'a str, &'a str) {
    ("(", ")")
}


