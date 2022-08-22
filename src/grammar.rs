use std::collections::HashMap;

pub struct Grammar{
    non_terminals: Vec<String>,
    terminals : Vec<String>,
    rules: HashMap<String, String>,
    start: String
}
