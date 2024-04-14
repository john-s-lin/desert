use std::sync::atomic::{AtomicU64, Ordering};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

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
        Doctor {
            id: ID.fetch_add(1, Ordering::SeqCst),
            interaction_time: 0,
            efficiency: 1.0,
            burnout_rate: rng.gen_range(0.0..0.2),
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
