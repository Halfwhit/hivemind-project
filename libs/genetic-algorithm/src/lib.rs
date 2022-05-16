pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn evolve<I>(&self, population &[I]) -> Vec<I> {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                todo!()
            })
            .collect()
    }
}

#[cfg(test)]
mod genetic_algorithm {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
