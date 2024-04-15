use std::sync::atomic::{AtomicU64, Ordering};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal, Uniform};

static ID: AtomicU64 = AtomicU64::new(0);

const SEED: u64 = 0;

#[derive(Debug)]
pub struct Doctor {
    id: u64,
    interaction_time: u64,
    efficiency: f32,
    burnout_rate: f32,
}

pub struct DoctorFactory;

impl DoctorFactory {
    fn create_doctor(rng: &mut impl Rng) -> Doctor {
        let efficiency_range = Normal::new(1.0, 0.2).unwrap();
        let burnout_range = Uniform::new(0.0, 0.1);
        Doctor {
            id: ID.fetch_add(1, Ordering::SeqCst),
            interaction_time: 0,
            efficiency: efficiency_range.sample(rng),
            burnout_rate: burnout_range.sample(rng),
        }
    }

    pub fn generate_vec_doctors(size: u64) -> Vec<Doctor> {
        let mut dr_vec = Vec::new();
        let mut rng = ChaCha8Rng::seed_from_u64(SEED);
        for _ in 0..size {
            let dr = DoctorFactory::create_doctor(&mut rng);
            dr_vec.push(dr);
        }
        dr_vec
    }
}
