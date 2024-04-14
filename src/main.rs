use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use doctor::Doctor;
const SEED: u64 = 0;
mod doctor;
fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    for _ in 0..10 {
        let doc = Doctor::new(rng.gen_range(0.0..0.2));
        println!("{:?}", doc);
    }
}
