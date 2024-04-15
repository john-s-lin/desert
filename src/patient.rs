use std::sync::atomic::{AtomicU64, Ordering};

use rand::Rng;
use rand_distr::{Distribution, Uniform};

static ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct Patient {
    id: u64,
    severity_score: u64,
    time_of_arrival: u64,
    time_waited: u64,
    time_to_treat: u64,
}

pub struct PatientFactory;

impl PatientFactory {
    pub fn create_patient(rng: &mut impl Rng, time_of_arrival: u64) -> Patient {
        let severity_range = Uniform::new(0, 100);
        let treatment_time_range = Uniform::new(15, 60);
        Patient {
            id: ID.fetch_add(1, Ordering::SeqCst),
            severity_score: severity_range.sample(rng),
            time_of_arrival,
            time_waited: 0,
            time_to_treat: treatment_time_range.sample(rng),
        }
    }
}
}
