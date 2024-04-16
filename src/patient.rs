use std::sync::atomic::{AtomicU64, Ordering};

use rand::Rng;
use rand_distr::{Distribution, Uniform};

static ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct Patient {
    pub id: u64,
    pub severity_score: u64,
    pub time_of_arrival: u64,
    pub time_waited: u64,
    pub time_to_treat: u64,
}

/// A minimized struct that takes Patient ID and position score that is stored in a BinaryHeap.
/// This way, the BinaryHeap does not need to store Patients, but only their IDs.
///
/// When a patient is occupied by a doctor, they are taken off the queue.
#[derive(Debug)]
pub struct PatientPosition {
    pub id: u64,
    pub position_score: f64,
}

impl PartialEq for PatientPosition {
    fn eq(&self, other: &Self) -> bool {
        // To account for float arithmetic, we'll assume equality when
        // the difference between two position_scores are less than 1E-5
        self.position_score - other.position_score < 1e-5
    }
}

impl Eq for PatientPosition {}

impl PartialOrd for PatientPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.position_score
                .partial_cmp(&other.position_score)
                .unwrap(),
        )
    }
}

impl Ord for PatientPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
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
