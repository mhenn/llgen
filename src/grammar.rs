use std::{
    collections::{HashMap, VecDeque},
    error, fmt,
    thread::AccessError,
};

pub struct Grammar<'a> {
    pub non_terminals: Vec<&'a str>,
    pub terminals: Vec<&'a str>,
    pub rules: HashMap<&'a str, Vec<Vec<&'a str>>>,
    pub start: &'a str,
}

pub enum NodeType {
    Terminal,
    NonTerminal,
}

#[derive(Debug)]
struct NoEntryError;

impl fmt::Display for NoEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Given Value is no Entry!")
    }
}

impl error::Error for NoEntryError {}

pub struct Node<'a> {
    pub value: &'a str,
    //pub parent: Option<Box<Node<'a>>>,
    pub visited: bool,
    pub typ: Option<NodeType>,
    pub path: Vec<&'a str>,
}

impl<'a> Grammar<'a> {
    pub fn check_membership(&self, value: &'a str) -> Result<NodeType, NoEntryError> {
        if self.non_terminals.contains(&value) {
            return Ok(NodeType::NonTerminal);
        } else if self.terminals.contains(&value) {
            return Ok(NodeType::Terminal);
        }
        Err(NoEntryError)
    }

    pub fn path_to_terminal(&self, value: &'a str) -> Result<Vec<&'a str>, NoEntryError> {
        let mut queue: VecDeque<Node> = VecDeque::new();
        let typ = self.check_membership(value)?;
        let mut el: Node;
        queue.push_front(Node {
            value,
            visited: false,
            typ: Some(typ),
            path: vec![],
        });
        while let Some(elem) = queue.pop_back() {
            println!("{}", elem.value);
            let mut path = elem.path.clone();
            path.push(elem.value.clone());
            if let Some(rules) = self.rules.get(elem.value) {
                for rule in rules {
                    for child in rule {
                        let typ = self.check_membership(child);
                        if typ.is_err() {
                            println!("{:?}", child);
                        }

                        queue.push_front(Node {
                            value: child,
                            visited: false,
                            typ: Some(typ.unwrap()),
                            path: path.clone(),
                        });
                    }
                }
            } else {
            }
        }
        Err(NoEntryError)
    }
}

pub fn get_test_grammar<'a>() -> Grammar<'a> {
    let mut map: HashMap<&str, Vec<Vec<&str>>> = HashMap::new();
    map.insert("<start>", vec![vec!["<expr>", "<op>", "<expr>"]]);
    map.insert(
        "<expr>",
        vec![vec!["<term>", "<op>", "<term>"], vec!["<term>"]],
    );
    map.insert("<op>", vec![vec!["+"], vec!["-"], vec!["/"], vec!["*"]]);
    map.insert("<term>", vec![vec!["x"], vec!["0"]]);
    Grammar {
        non_terminals: vec!["<start>", "<expr>", "<op>", "<term>"],
        terminals: vec!["+", "-", "/", "*", "x", "0"],
        rules: map,
        start: "<start>",
    }
}

#[test]
fn bfs_grammar() {
    let grammar = get_test_grammar();
    let path = grammar.path_to_terminal("<start>");
    print!("{:?}", path);
    assert!(path.is_ok());
}

//
//
//#[test]
//fn calculate_rule_count_vals(){
//    let mut map: HashMap<String, Vec<Vec<String>>> = HashMap::new();
//    map.insert(String::from("a") , vec![vec![String::from("a")], vec![String::from("Giraffe")]]);
//    map.insert(String::from("b") , vec![ vec![String::from("Giraffe")]]);
//    map.insert(String::from("c") , vec![vec![String::from("a")], vec![String::from("Giraffe")], vec![String::from("Girfe")], vec![String::from("Giraff")]]);
//    let g = Grammar::new(vec![], vec![], map.clone(), String::from("Affe"));
//    for (x,&y) in g.rule_count.iter(){
//        assert!(y == map.get(x).unwrap().len())
//    }
//}
