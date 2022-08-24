#[derive(Default)]
pub struct Individual {
    pub chromosome: Vec<u8>,
    pub fitness: f64,
}

#[derive(Default)]
pub struct Generation {
    pub individuals: Vec<Individual>,
    pub count: i32,
}

impl Generation {
    pub fn populate(&mut self, func: fn(i32, i32, i32) -> Generation) {
        let gen = func(1, 10, 100);
        self.individuals = gen.individuals;
    }
}
