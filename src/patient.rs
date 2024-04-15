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

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::PatientFactory;

    const SEED: u64 = 123;

    #[test]
    fn test_create_patient() {
        let mut rng = ChaCha8Rng::seed_from_u64(SEED);
        let pt = PatientFactory::create_patient(&mut rng, 10);

        assert!((0..100).contains(&pt.severity_score));
        assert_eq!(*&pt.time_waited, 0);
        assert!((15..60).contains(&pt.time_to_treat));
    }
}
