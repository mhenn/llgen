use crate::grammar::*;
use std::collections::HashMap;

pub struct Interpreter<'a>{
    pos: usize,
    pub word: Vec<&'a str>,

}


fn handle_derivation<'a>( codon: &u8, key: &str, rules: &HashMap<&'a str, Vec<Vec<&'a str>>>) -> Option<Vec<&'a str>>{
    let codon: usize = *codon as usize;
    println!("key: {key}");
    if let Some(value) = rules.get(&key){
        let len: usize = value.len();
        return Some(value[codon % len].clone())
    } else{
        None
    }
}


impl<'a> Interpreter<'a>{

    pub fn new(start: &'a str) -> Self{
        Interpreter { pos: 0, word: vec!["<start>"]}// vec![start] }
    }
    fn get_new_nt_pos(&self, non_terminals: &Vec<&str>) -> usize{
        let word = &self.word;
        word.into_iter().fold((0,false), |mut acc, x|
                              {
                                  if acc.1{
                                      return acc
                                  }
                                  if non_terminals.contains(&x) {
                                      acc.1 = true;
                                  } else {
                                      acc.0 += 1;
                                  }
                                  acc
                              }
                             ).0
    }


    fn derive(&mut self,codon: u8, rules: &HashMap<&'a str, Vec<Vec<&'a str>>>) {
        //let word = &self.word;
        let pos = self.pos;
        println!("{codon}");
        if let Some(value) = &self.word.get(pos){
            let derivative = handle_derivation(&codon, value, &rules);
            if let Some(update) = derivative{
                //self.word.remove(self.pos);
                self.word.splice(pos.. &pos+1, update);
            }
        } // Return Error
    }

    pub fn interpret(&mut self, chromosome:  Vec<u8>, rules: &HashMap<&'a str, Vec<Vec<&'a str>>>, non_terminals: &Vec<&str>) {//-> Result<String, InterpeterError>{
        println!("NT: {:?}", rules);
        for codon in chromosome {
            println!("word: {:?}", self.word);
            println!("pos: {}", self.pos);
            self.derive(codon, rules);
            self.pos = self.get_new_nt_pos(non_terminals);
        }
    }
}

    //TODO  currently not needed
    //    //
    //    use std::fmt;
    //
    //
    //#[derive(Debug, Clone)]
    //    struct InterpeterError;
    //
    //    impl fmt::Display for InterpeterError  {
    //        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //            write!(f, "Word couldn't be fully derived!")
    //        }
    //    }
    //
    //

    fn get_test_grammar<'a>() -> Grammar<'a>{
        let mut  map: HashMap<&str, Vec<Vec<&str>>> = HashMap::new();
        map.insert("<start>", vec![vec!["<expr>","<op>","<expr>"]]);
        map.insert("<expr>", vec![vec!["<term>","<op>","<term>"], vec!["<term>"]]);
        map.insert("<op>", vec![vec!["+"], vec!["-"], vec!["/"], vec!["*"]]);
        map.insert("<term>", vec![vec!["x"], vec!["0"]]);
        Grammar { non_terminals: vec!["<start>", "<expr>", "<op>", "<term>"], terminals: vec!["+", "-", "/", "*", "x", "0"], rules: map, start:"" }
    }

#[test]
    fn interpreting_standard_grammar(){
        let grammar = get_test_grammar();
        let mut inter = Interpreter::new(grammar.start );
        inter.interpret(vec![13,4,9,33,16,14,3,28], &grammar.rules, &grammar.non_terminals);
        println!("{:?}",inter.word);
        assert!(false);
    }

