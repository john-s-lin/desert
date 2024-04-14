use std::sync::atomic::{AtomicU64, Ordering};

static ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct Doctor {
    id: u64,
    interaction_time: u64,
    efficiency: f32,
    burnout_rate: f32,
}

impl Doctor {
    pub fn new(burnout_rate: f32) -> Self {
        Doctor {
            id: ID.fetch_add(1, Ordering::SeqCst),
            interaction_time: 0,
            efficiency: 1.0,
            burnout_rate,
        }
    }
}
