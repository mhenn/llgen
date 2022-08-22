#[derive(Default)]
pub struct Individual{
    pub chromosome: Vec<u8>,
    pub fitness: f64
}

pub struct Generation{
    pub individuals: Vec<Individual>,
    pub count: i32,
}

