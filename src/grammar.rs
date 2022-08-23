use std::collections::HashMap;



pub struct Grammar<'a>{
   pub non_terminals: Vec<&'a str>,
   pub terminals : Vec<&'a str>,
   pub rules: HashMap<&'a str, Vec<Vec<&'a str>>>,
   pub start: &'a str,
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
