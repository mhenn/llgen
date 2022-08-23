
use std::collections::HashMap;


pub struct Interpreter<'a>{
    pos: usize,
    word: Vec<&'a str>,

}

impl<'a> Interpreter<'a>{

    pub fn new(start: &'a str) -> Self{
        Interpreter { pos: 0, word: vec![start] }
    }

    fn handle_derivation(&mut self, codon: &u8, key: &str, rules: &HashMap<&'a str, Vec<Vec<&'a str>>>) -> Option<Vec<&str>>{
        let codon: usize = *codon as usize;
        if let Some(value) = rules.get(&key){
            let len: usize = value.len();
            Some(value[codon % len ].clone())
        } else{
            None
        }
    }

    fn get_new_nt_pos(&mut self, non_terminals: &Vec<&str>) -> usize{
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


    fn derive(&mut self, codon: &u8, rules: &HashMap<&'a str, Vec<Vec<&'a str>>>) -> Vec<&str>{
        let mut  word = self.word.clone();
        let pos = self.pos;
        if let Some(value) = word.get(pos ){
            let derivative = self.handle_derivation(codon, value, &rules);
            if let Some(update) = derivative{

                //self.word.remove(self.pos);
                word.splice(pos.. pos+1, update);
            }
        } // Return Error
        word
    }

    pub fn interpret(&mut self, chromosome:  Vec<u8>, rules: &HashMap<&'a str, Vec<Vec<&'a str>>>, non_terminals: &Vec<&str>) {//-> Result<String, InterpeterError>{
                                                                                                                       //
        for codon in chromosome {
            self.derive(&codon, rules);
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

//fn get_test_grammar<'a>() -> Grammar<'a>{
//    let  map: HashMap<&str, Vec<Vec<&str>>> = HashMap::new();
//    map.insert("<start>", vec![vec!["<expr>","<op>","<expr>"]]);
//    map.insert("<expr>", vec![vec!["<term>","<op>","<term>"], vec!["<term>"]]);
//    map.insert("<op>", vec![vec!["+","-","/", "*"]]);
//    map.insert("<term>", vec![vec!["x","0"]]);
//   Grammar { non_terminals: vec!["<start>", "<expr>", "<op>", "<term>"], terminals: vec!["+", "-", "/", "*", "x", "0"], rules: map, start:"" }
//}
//
//#[test]
//    fn interpreting_standard_grammar(){
//        let grammar = get_test_grammar();
//        let inter = Interpreter::new(grammar );
//        inter.interpret(vec![13,4,9,33,16,14,3,28]);
//        println!("{:?}",inter.word);
//        assert!(false);
//    }
//
