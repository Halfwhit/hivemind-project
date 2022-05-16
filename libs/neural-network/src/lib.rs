use rand::prelude::*;

pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Clone)]
struct Layer {
    neurons: Vec<Neuron>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Clone)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Network {
    pub fn random(
        rng: &mut dyn rand::RngCore,
        layers: &[LayerTopology]
    ) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

impl Layer {
    pub fn random(
            rng: &mut dyn rand::RngCore,
            input_neurons: usize, 
            output_neurons: usize
        ) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(rng, input_neurons))
            .collect();

        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

impl Neuron {
    pub fn random(
        rng: &mut dyn rand::RngCore,
        output_size: usize
    ) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }
}

#[cfg(test)]
mod neural_network {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use approx::assert_relative_eq;

    mod random {
        use super::*;

        #[test]
        fn neuron() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 3);

            assert_relative_eq!(neuron.bias, -0.6255188);

            assert_relative_eq!(neuron.weights.as_slice(),
            [0.67383957, 0.8181262, 0.26284897].as_slice()
            );
        }

        #[test]
        fn layer() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer = Layer::random(&mut rng, 3, 2);

            assert_relative_eq!(layer.neurons[0].bias, -0.6255188);

            assert_relative_eq!(layer.neurons[0].weights.as_slice(), [0.67383957, 0.8181262, 0.26284897].as_slice());
        }

        #[test]
        fn network() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let network = Network::random(&mut rng, &[
                LayerTopology { neurons: 3 },
                LayerTopology { neurons: 2 },
                LayerTopology { neurons: 1 },
            ]);

            assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);

            assert_relative_eq!(network.layers[0].neurons[0].weights.as_slice(), [0.67383957, 0.8181262, 0.26284897].as_slice());
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn neuron() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };
        
            assert_relative_eq!(
                neuron.propagate(&[-10.0, -10.0]),
                0.0,
            );
        
            assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
            );

            // 1.15
        }

        #[test]
        fn layer() {
            let neurons = vec![
                Neuron {bias: 0.0, weights: vec![0.1, 0.2, 0.3]}, 
                Neuron {bias: 0.0, weights: vec![0.4, 0.5, 0.6]}
            ];

            let layer = Layer { neurons: neurons.clone() };

            let inputs = &[-0.5, 0.0, 0.5];

            let actual = layer.propagate(inputs.to_vec());
            let expected = vec![neurons[0].propagate(inputs), neurons[1].propagate(inputs)];

            assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }

        #[test]
        fn network() {
            let layers = ( 
                Layer {neurons: vec![
                    Neuron {bias: 0.0, weights: vec![-0.5, -0.4, -0.3]}, 
                    Neuron {bias: 0.0, weights: vec![-0.2, -0.1, 0.0]}
                ]},
                Layer {neurons: vec![Neuron {bias: 0.0, weights: vec![-0.5, 0.5]}] }
            );

            let network = Network { layers: vec![layers.0.clone(), layers.1.clone()]};

            let actual = network.propagate(vec![0.5, 0.6, 0.7]);
            let expected = layers.1.propagate(layers.0.propagate(vec![0.5, 0.6, 0.7]));

            assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }
}
