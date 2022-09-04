#![allow(dead_code, unused)]
use crate::grammar::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    TooManyCodonsError,
    TooFewCodonsError,
}

impl std::error::Error for InterpreterError {}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::TooFewCodonsError => {
                write!(f, "To many Codons to build a valid tree!")
            }
            InterpreterError::TooManyCodonsError => write!(f, "To few Codons to build valid tree!"),
        }
    }
}

fn handle_derivation<'a>(
    codon: &u8,
    key: &str,
    rules: &HashMap<&'a str, Vec<Vec<&'a str>>>,
) -> Option<Vec<&'a str>> {
    let codon: usize = *codon as usize;
    println!("key: {key}");
    if let Some(value) = rules.get(&key) {
        let len: usize = value.len();
        Some(value[codon % len].clone())
    } else {
        None
    }
}

fn get_new_nt_pos(word: &Vec<&str>, non_terminals: &Vec<&str>) -> Option<usize> {
    let new_pos = word.iter().fold((0, false), |mut acc, x| {
        if acc.1 {
            return acc;
        }
        if non_terminals.contains(x) {
            acc.1 = true;
        } else {
            acc.0 += 1;
        }
        acc
    });
    if new_pos.1 {
        Some(new_pos.0)
    } else {
        None
    }
}

fn derive<'a>(
    pos: usize,
    word: &Vec<&'a str>,
    codon: u8,
    rules: &HashMap<&'a str, Vec<Vec<&'a str>>>,
) -> Vec<&'a str> {
    let mut word = word.clone();
    println!("{codon}");
    if let Some(value) = &word.get(pos) {
        let derivative = handle_derivation(&codon, value, &rules);
        if let Some(update) = derivative {
            word.splice(pos..&pos + 1, update);
        }
    } // Return Error
    word
}

pub fn interpret<'a>(
    chromosome: &Vec<u8>,
    grammar: &'a Grammar,
) -> Result<Vec<&'a str>, InterpreterError> {
    //-> Result<String, InterpeterError>{
    let mut pos;
    let mut word: Vec<&str> = vec![grammar.start];
    println!("{:?}", grammar.start);
    for codon in chromosome {
        println!("{:?}", word);
        if let Some(new_pos) = get_new_nt_pos(&word, &grammar.non_terminals) {
            pos = new_pos;
        } else {
            return Err(InterpreterError::TooManyCodonsError);
        }
        word = derive(pos, &word, *codon, &grammar.rules);
    }

    if get_new_nt_pos(&word, &grammar.non_terminals).is_some() {
        return Err(InterpreterError::TooFewCodonsError);
    }

    Ok(word)
}
