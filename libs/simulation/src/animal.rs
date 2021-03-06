use crate::*;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Animal {
    crate position: na::Point2<f32>,
    crate rotation: na::Rotation2<f32>,
    crate speed: f32,
    crate eye: Eye,
    crate brain: nn::Network
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();

        let brain = nn::Network::random(
            rng, 
            &[
                nn::LayerTopology { neurons: eye.cells() },
                nn::LayerTopology { neurons: 2 * eye.cells() },
                nn::LayerTopology { neurons: 2 }
            ]
        );
        
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}