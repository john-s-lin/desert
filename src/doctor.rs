use std::sync::atomic::{AtomicU64, Ordering};

use rand::Rng;
use rand_distr::{Distribution, Uniform};

use crate::patient::Patient;

static ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct Doctor {
    id: u64,
    interaction_time: u64,
    efficiency: f32,
    burnout_rate: f32,
}

impl Doctor {
    pub fn update_interaction_time(&mut self, patient: Patient) {
        self.interaction_time +=
            (patient.time_to_treat as f64 / self.efficiency as f64).round() as u64;
    }

    pub fn update_efficiency(&mut self) {
        self.efficiency -= self.burnout_rate;
    }
}

pub struct DoctorFactory;

impl DoctorFactory {
    /// Creates a Doctor struct with randomly generated values for `efficiency` and `burnout_rate`
    ///
    /// # Examples
    ///
    /// ```
    /// // rng is a pre-initialized random number generator that is passed in as an argument
    /// let dr = create_doctor(&mut rng);
    ///
    /// assert_eq!(*&dr.interaction_time, 0);
    /// assert!((0.8..1.2).contains(&dr.efficiency));
    /// assert!((0.0..0.1).contains(&dr.burnout_rate));
    /// ```
    fn create_doctor(rng: &mut impl Rng) -> Doctor {
        let efficiency_range = Uniform::new(0.8, 1.2);
        let burnout_range = Uniform::new(0.0, 0.1);
        Doctor {
            id: ID.fetch_add(1, Ordering::SeqCst),
            interaction_time: 0,
            efficiency: efficiency_range.sample(rng),
            burnout_rate: burnout_range.sample(rng),
        }
    }

    /// Generates a vector of Doctor structs of given size
    ///
    /// # Examples
    ///
    /// ```
    /// let doctor_vec = generate_vec_doctors(10, rng)
    ///
    /// assert_eq!(doctor_vec.len(), 10);
    /// ```
    pub fn generate_vec_doctors(size: u64, rng: &mut impl Rng) -> Vec<Doctor> {
        let mut dr_vec = Vec::new();
        for _ in 0..size {
            let dr = DoctorFactory::create_doctor(rng);
            dr_vec.push(dr);
        }
        dr_vec
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::DoctorFactory;

    const SEED: u64 = 123;

    #[test]
    fn test_create_doctor() {
        let mut rng = ChaCha8Rng::seed_from_u64(SEED);
        let dr = DoctorFactory::create_doctor(&mut rng);

        assert_eq!(*&dr.interaction_time, 0);
        assert!((0.8..1.2).contains(&dr.efficiency));
        assert!((0.0..0.1).contains(&dr.burnout_rate));
    }

    #[test]
    fn test_generate_vec_doctors() {
        let mut rng = ChaCha8Rng::seed_from_u64(SEED);
        let dr_vec = DoctorFactory::generate_vec_doctors(10, &mut rng);

        assert_eq!(dr_vec.len(), 10);
    }
}
