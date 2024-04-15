use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use doctor::DoctorFactory;
use patient::PatientFactory;
use queue::SchedulerSingleQueue;

mod doctor;
mod patient;
mod queue;

const SEED: u64 = 0;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let vec_docs = DoctorFactory::generate_vec_doctors(10, &mut rng);
    dbg!(&vec_docs);

    let pt = PatientFactory::create_patient(&mut rng, 20);
    dbg!(&pt);

    let mut queue = SchedulerSingleQueue::new();
    dbg!(&queue);
    let val = queue.calculate_position_score(&pt);
    dbg!(val);
    queue.push(&pt);
    dbg!(&queue);
}
