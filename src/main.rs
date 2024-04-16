use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use doctor::DoctorFactory;
use patient::{Patient, PatientFactory};
use queue::SchedulerSingleQueue;

mod doctor;
mod patient;
mod queue;

const SEED: u64 = 0;

// Set 1 tick = 1 min. Assume shift length = 8h = 480 min
const MAX_TICKS: u64 = 480;

// Set probability that patient will be entered into queue at 0.2 every tick cycle
const PATIENT_PROBABILITY: f32 = 0.2;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    // let vec_docs = DoctorFactory::generate_vec_doctors(10, &mut rng);
    let mut vec_patients: Vec<Patient> = Vec::new();
    let mut triage_queue = SchedulerSingleQueue::new();

    for tick in 0..MAX_TICKS {
        // // Update time-waited for all patients and update position score
        // vec_patients
        //     .iter()
        //     .map(|&mut pt| pt.increment_time_waited());

        if rng.gen_range(0.0..1.0) < PATIENT_PROBABILITY {
            let patient = PatientFactory::create_patient(&mut rng, tick);
            triage_queue.push(&patient);
            vec_patients.push(patient);
        }
    }
}
