use std::collections::VecDeque;

use crate::grammar::*;

#[derive(Clone)]
pub struct GNode<T> {
    pub value: Vec<T>,
    //    pub  ids: Vec<u8>,
    pub edges: Vec<Edge<T>>,
}

#[derive(Clone)]
pub struct Edge<T> {
    pub value: T,
    pub end: GNode<T>,
}

impl<T> GNode<T> {
    fn add_edge_to(&mut self, end: GNode<T>, value: T) {
        self.edges.push(Edge { value, end });
    }
}

fn only_terminals(grammar: &Grammar, ls: &Vec<&str>) -> bool {
    let contains = ls
        .iter()
        .any(|x| matches!(grammar.check_membership(x), Ok(NodeType::NonTerminal)));
    !contains
}

pub fn smallest_word<'a>(
    grammar: &'a Grammar,
    word: Vec<&'a str>,
) -> Result<Vec<&'a str>, NoEntryError> {
    let mut queue: VecDeque<(Vec<usize>, Vec<&str>)> = VecDeque::new();
    queue.push_front((vec![], word));


    while let Some((mut decisions,  current_word)) = queue.pop_back() {
        println!("{:?}", current_word);
        let pos = grammar.get_next_nt_in(&current_word);
        if pos.is_none() {
            return Ok(current_word);
        }
        let pos = pos.unwrap();
        for (i, rule) in grammar
            .rules
            .get(current_word[pos])
            .unwrap()
            .iter()
            .enumerate()
        {
            let mut new_word = current_word.clone();
            new_word.splice(pos..(pos+1), rule[..].to_vec());
            decisions.push(i);
            queue.push_front((decisions.clone(), new_word));
        }
    }
            println!("WHAAAT");

    Err(NoEntryError)
}

#[test]
fn word_with_just_terminals(){
    let word = vec!["<root>","pickup","</root>"];
    let grammar = get_bt_grammar();
    assert!(only_terminals(&grammar, &word));
}

#[test]
fn words_with_non_terminals(){
    let grammar = get_bt_grammar();
    let word = vec!["<root>","NL","</root>"];

    assert_eq!(only_terminals(&grammar, &word), false);
}

#[test]
fn smallest_word_from_start() {
    let word = vec!["<root>","NL" ,"</root>"];
    let grammar = get_bt_grammar();

    let res = smallest_word(&grammar, word);
     assert!(res.is_ok());
}
