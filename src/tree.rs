use std::collections::VecDeque;

use crate::grammar::*;

#[derive(Clone)]
pub struct GNode<T>{
    pub  value: Vec<T>,
    //    pub  ids: Vec<u8>,
    pub  edges: Vec<Edge<T>>,
}

#[derive(Clone)]
pub struct Edge<T>{
    pub  value: T,
    pub  end: GNode<T>,
}

impl<T> GNode<T>{

    fn add_edge_to(&mut self,end: GNode<T>, value: T){
        self.edges.push(Edge{value, end });
    }

}

fn only_terminals(grammar: &Grammar, ls: &Vec<&str>) -> bool{
    let contains =  ls.iter().any(|x| matches!(grammar.check_membership(x), Ok(NodeType::NonTerminal)));
    !contains
}


pub fn smallest_word<'a>(grammar: &'a Grammar, word: Vec<&'a str>) -> Result<Vec<&'a str>, NoEntryError>{

    let mut queue: VecDeque<(Vec<usize>, Vec<&str>)> = VecDeque::new();
    queue.push_front((vec![], word));

    while let Some((mut decisions, mut current_word)) = queue.pop_front()  {
        let pos = grammar.get_next_nt_in(&current_word).ok_or(NoEntryError)?;
            if only_terminals(grammar, &current_word){
                return Ok(current_word)
            }
        for (i,rule) in grammar.rules.get(current_word[pos]).unwrap().iter().enumerate(){
            current_word.splice(pos..pos, rule[..].to_vec());
            decisions.push(i);
        }
    }

    Err(NoEntryError)

}



