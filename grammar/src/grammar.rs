#![allow(dead_code, unused)]
use std::{
    collections::{HashMap, VecDeque},
    error, fmt,
    hash::Hash,
};

use crate::tree::*;

pub struct Grammar<'a> {
    pub non_terminals: Vec<&'a str>,
    pub terminals: Vec<&'a str>,
    pub rules: HashMap<&'a str, Vec<Vec<&'a str>>>,
    pub start: &'a str,
    pub non_terms: HashMap<&'a str, bool>,
}

pub enum NodeType {
    Terminal,
    NonTerminal,
}

#[derive(Debug)]
pub struct NoEntryError;

impl fmt::Display for NoEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Given Value is no Entry!")
    }
}

impl error::Error for NoEntryError {}

impl<'a> Grammar<'a> {
    pub fn check_membership(&self, value: &'a str) -> Result<NodeType, NoEntryError> {
        if self.non_terminals.contains(&value) {
            return Ok(NodeType::NonTerminal);
        } else if self.terminals.contains(&value) {
            return Ok(NodeType::Terminal);
        }
        Err(NoEntryError)
    }

    fn add_rules_to_queue(&self, value: &'a str, queue: &mut VecDeque<(u8, Vec<&'a str>)>) {
        let i: u8 = 0;
        for rule in self.rules.get(value).unwrap() {
            queue.push_front((i, rule.clone()))
        }
    }

    pub fn get_next_nt_in(&self, word: &Vec<&str>) -> Option<usize> {
        for (i, el) in word.iter().enumerate() {
            if self.non_terms.get(el).is_some() {
                return Some(i);
            }
        }
        None
    }

    //    pub fn get_next_nt_in(&self, word: &Vec<&str>) -> Option<usize> {
    //        if let Some(nt) = word.iter().find(|x| self.non_terminals.contains(x)) {
    //            return word.iter().position(|x| x == nt);
    //        }
    //        None
    //    }

    pub fn fill_word(&self, word: Vec<&str>) {}

    // build an "infinite" tree, the value is the resulting word after the rule execution
}

////////////// TEST STUFF //////////////////

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
        non_terms: HashMap::new(),
    }
}

pub fn get_bt_grammar<'a>() -> Grammar<'a> {
    let mut map = HashMap::new();
    map.insert("ROOT", vec![vec!["<root>", "NL", "</root>"]]);
    map.insert("NL", vec![vec!["LOCF"], vec!["LOCF", "NL"]]);
    map.insert("LOCF", vec![vec!["L"], vec!["CF"]]);
    map.insert("L", vec![vec!["pickup"]]);
    map.insert(
        "CF",
        vec![
            vec!["<pol>", "LOCF", "</pol>"],
            vec!["<seq>", "NL", "</seq>"],
            vec!["<fall>", "NL", "</fall>"],
            vec!["<par>", "NL", "</par>"],
        ],
    );
    map.insert("ACT", vec![vec!["pickup"], vec!["putdown"]]);

    let mut nt = HashMap::new();
    nt.insert("ROOT", true);
    nt.insert("NL", true);
    nt.insert("LOCF", true);
    nt.insert("L", true);
    nt.insert("CF", true);
    nt.insert("ACT", true);

    Grammar {
        non_terminals: vec!["ROOT", "NL", "LOCF", "L", "CF", "ACT"],
        terminals: vec![
            "<root>", "<seq>", "<fall>", "<par>", "<pol>", "</root>", "</seq>", "</fall>",
            "</par>", "</pol>", "pickup", "putdown",
        ],
        rules: map,
        start: "ROOT",
        non_terms: nt,
    }
}

//use std::time::{Duration, Instant};
//let start = Instant::now();
//let duration = start.elapsed();
//println!("Time elapsed in expensive_function() is: {:?}", duration);
