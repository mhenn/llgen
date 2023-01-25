use crate::{constraints::get_nodes, settings::Settings, nodes::Node};

#[test]
fn get_grow() {
    let nodes = get_nodes();
    let mut settings = Settings::new().unwrap();
    settings.population.tree_depth = 3;
    settings.population.tree_width = 3;
    //    let expr = gen_rnd_expr_tree(&nodes, &get_delim(), &settings, false);
    //    println!("{:?}", expr);
}


pub fn node_to_xml_string<'a, T>(node: &Node<T>, delimeter: &(&'a str, &'a str, &'a str) ) -> String
where
    T: Default + Copy,
    String: From<T>
{
    let val : String = node.value.try_into().unwrap();
    let mut expr: String = "".to_owned();
    expr.push_str(delimeter.0);
    expr.push_str(&val);

    if node.children.is_empty() {
        expr.push_str(delimeter.1);
    }
    expr.push_str(delimeter.2);

    for sub_t in node.children.iter() {
        expr.push_str( &node_to_xml_string(sub_t, delimeter).to_string());
    }

    if node.children.is_empty() {
        return expr;
    }
    expr.push_str(delimeter.0);
    expr.push_str(delimeter.1);
    expr.push_str(&val);
    expr.push_str(delimeter.2);

    expr
}


