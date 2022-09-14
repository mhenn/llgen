#![allow(dead_code, unused)]

pub struct Generation<T> {
    pub population: Vec<Individual<T>>,
}

pub struct Individual<T> {
    pub chromosome: T,
    pub fitness: f64,
    pub id: usize,
}

pub fn to_xml<'a>(value: Vec<&'a str>, delims: &(&'a str, &'a str, &'a str)) -> Vec<&'a str> {
    let (delim_l, closing, delim_r) = delims;

    if value.is_empty() {
        return vec![];
    }

    let mut xml: Vec<&str> = vec![];
    let mut elem: &str = "<";
    let mut elem_end: &str = "</";
    let (mut res, node) = match value.as_slice() {
        ["(", node, middle @ .., ")"] => (to_xml(middle.to_vec(), delims), *node),
        [node, rest @ ..] => {
            let mut el = vec![*delim_l, node, closing, *delim_r];
            xml.append(&mut el);
            xml.append(&mut to_xml(rest.to_vec(), delims));
            return xml;
        }
        _ => (vec![], ""),
    };

    if node.is_empty() {
        return vec![];
    }

    let mut el = vec![*delim_l, node, *delim_r];
    xml.append(&mut el);
    let mut el = vec![*delim_l, *closing, node, *delim_r];
    xml.append(&mut res);
    xml.append(&mut el);
    xml
}

//
//impl<T> Individual<T>{
//
//
//    pub fn to_xml(&self, (delim_l, delim_r): &(T,T)) -> Vec<T>
//    where T: Clone + Copy
//    {
//        let mut xml:  Vec<T> = vec![];
//        let mut tail: Vec<T> = vec![];
//        let mut value = self.chromosome.clone();
//        while !value.is_empty(){
//            let mut res:Vec<T> = match value.clone().as_slice(){
//                [delim_l, node, middle @ .., delim_r] => {
//                    value = middle.to_vec();
//                    vec![*node]},
//                _ => vec![]
//            };
//            let mut l_del = vec![*delim_l];
//            let mut r_del = vec![*delim_l, *delim_l];
//            l_del.append(&mut res);
//            l_del.push(*delim_r);
//            l_del.append(&mut res);
//            l_del.push(*delim_r);
//            xml.append(&mut l_del);
//            tail.append(&mut res);
//        }
//        xml.append(&mut tail);
//        xml
//    }
//}
//
//
