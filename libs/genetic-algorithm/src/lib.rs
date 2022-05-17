#![feature(type_alias_impl_trait)]

use rand::prelude::*;
use std::ops::Index;

pub trait Individual {
    fn chromosome(&self) -> &Chromosome;
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}
impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S) -> Self {
        Self { selection_method }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();
                // Crossover
                // Mutation
                // Convert `Chromosome` back into `Individual`
                todo!()
            })
            .collect()
    }
}

pub struct RouletteWheelSelection;
impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}
impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("Empty population")
    }
}

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}
impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}
impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}
impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}
impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

#[derive(Clone, Debug)]
pub struct UniformCrossover;
impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}
impl CrossoverMethod for UniformCrossover

#[cfg(test)]
mod genetic_algorithm {
    use super::*;

    mod selection {
        use super::*;
        use rand_chacha::ChaCha8Rng;
        use std::collections::BTreeMap;

        #[cfg(test)]
        #[derive(Clone, Debug)]
        pub struct TestIndividual {
            fitness: f32,
        }

        #[cfg(test)]
        impl TestIndividual {
            pub fn new(fitness: f32) -> Self {
                Self { fitness }
            }
        }

        #[cfg(test)]
        impl Individual for TestIndividual {
            fn chromosome(&self) -> &Chromosome {
                panic!("Not supported for TestIndividual")
            }
            fn fitness(&self) -> f32 {
                self.fitness
            }
        }

        #[test]
        fn roulette_selection() {
            let method = RouletteWheelSelection::new();
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let population = vec![
                TestIndividual::new(2.0),
                TestIndividual::new(1.0),
                TestIndividual::new(4.0),
                TestIndividual::new(3.0),
            ];

            let actual_histogram: BTreeMap<i32, _> = (0..1000)
                .map(|_| method.select(&mut rng, &population))
                .fold(Default::default(), |mut histogram, individual| {
                    *histogram.entry(individual.fitness() as _).or_default() += 1;

                    histogram
                });

            let expected_histogram = maplit::btreemap! {
                1 => 98,
                2 => 202,
                3 => 278,
                4 => 422,
            };

            assert_eq!(actual_histogram, expected_histogram);
        }
    }

    mod chromosomes {
        use super::*;

        fn chromosome() -> Chromosome {
            Chromosome {
                genes: vec![3.0, 1.0, 2.0],
            }
        }

        #[test]
        fn len() {
            assert_eq!(chromosome().len(), 3);
        }

        #[test]
        fn iter() {
            let chromosome = chromosome();
            let genes: Vec<_> = chromosome.iter().collect();

            assert_eq!(genes.len(), 3);
            assert_eq!(genes[0], &3.0);
            assert_eq!(genes[1], &1.0);
            assert_eq!(genes[2], &2.0);
        }

        #[test]
        fn iter_mut() {
            let mut chromosome = chromosome();

            chromosome.iter_mut().for_each(|gene| {
                *gene *= 10.0;
            });

            let genes: Vec<_> = chromosome.iter().collect();

            assert_eq!(genes.len(), 3);
            assert_eq!(genes[0], &30.0);
            assert_eq!(genes[1], &10.0);
            assert_eq!(genes[2], &20.0);
        }

        #[test]
        fn index() {
            let chromosome = chromosome();

            assert_eq!(chromosome[0], 3.0);
            assert_eq!(chromosome[1], 1.0);
            assert_eq!(chromosome[2], 2.0);
        }

        #[test]
        fn from_iter() {
            let chromosome: Chromosome = vec![3.0, 1.0, 2.0].into_iter().collect();

            assert_eq!(chromosome[0], 3.0);
            assert_eq!(chromosome[1], 1.0);
            assert_eq!(chromosome[2], 2.0);
        }
    }
}
