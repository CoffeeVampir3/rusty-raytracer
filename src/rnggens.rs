use rand::{distributions::{DistIter, Uniform}, prelude::SmallRng, SeedableRng, Rng};
use rand_distr::{Distribution, StandardNormal};

type v3 = bevy_math::DVec3;

pub struct UniRngGen {
    generator : DistIter<Uniform::<f64>, SmallRng, f64>
}

impl UniRngGen {
    pub fn new() -> Self {
        let rng = SmallRng::from_entropy();

        let uni_dist = Uniform::<f64>::new_inclusive(0.0, 1.0);
        Self {
            generator: rng.sample_iter(uni_dist),
        }
    }

    pub fn gen(&mut self) -> f64 {
        //SAFETY:: I have no idea if this is actually okay. #yolo #swag
        self.generator.next().unwrap()
    }

    fn random_in_unit_sphere_rejection(&mut self) -> v3 {
        loop {
            let x = self.gen();
            let y = self.gen();
            let z = self.gen();
            let vec = v3::new(x, y, z);
            if vec.length_squared() > 1.0 {
                continue;
            }
            return vec
        }
    }
    
    fn random_in_unit_hemisphere_rejection(&mut self, normal:v3) -> v3 {
        let rius = self.random_in_unit_sphere_rejection().normalize_or_zero();
        if v3::dot(rius, normal) > 0.0 {
            return rius
        }
        -rius
    }
}

pub struct NormRngGen {
    generator : DistIter<StandardNormal, SmallRng, f64>
}

impl NormRngGen {
    pub fn new() -> Self {
        let rng = SmallRng::from_entropy();

        let norm_dist = rand_distr::StandardNormal;
        Self {
            generator: norm_dist.sample_iter(rng),
        }
    }

    pub fn gen(&mut self) -> f64 {
        //SAFETY:: I have no idea if this is actually okay. #yolo #swag
        self.generator.next().unwrap()
    }

    pub fn random_in_unit_sphere_from_norm(&mut self) -> v3 {
        let x = self.gen();
        let y = self.gen();
        let z = self.gen();

        v3::new(x,y,z).normalize_or_zero()
    }
    
    pub fn random_in_unit_hemisphere_from_norm(&mut self, normal:v3) -> v3 {
        let rius = NormRngGen::random_in_unit_sphere_from_norm(self);
        if v3::dot(rius, normal) > 0.0 {
            return rius
        }
        -rius
    }
}
