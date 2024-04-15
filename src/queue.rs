use std::collections::BinaryHeap;

use crate::patient::{Patient, PatientPosition};

const SEVERITY_RATIO: f64 = 0.5;
const TIME_WAITED_RATIO: f64 = 0.3;
const SHORTEST_JOB_RATIO: f64 = 0.2;

#[derive(Debug)]
pub struct SchedulerSingleQueue {
    queue: BinaryHeap<PatientPosition>,
}

impl SchedulerSingleQueue {
    pub fn new() -> SchedulerSingleQueue {
        SchedulerSingleQueue {
            queue: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, patient: &Patient) {
        let pt_score = self.calculate_position_score(&patient);
        self.queue.push(PatientPosition {
            id: *&patient.id,
            position_score: pt_score,
        });
    }

    /// Calculates position score given a reference to a Patient.
    /// Uses score to determine position in priority queue.
    ///
    /// # Examples
    ///
    /// ```
    /// let pt = Patient {
    ///     id: 1,
    ///     severity_score: 50,
    ///     time_of_arrival: 0,
    ///     time_waited: 30,
    ///     time_to_treat: 18,
    /// }
    /// assert!((self.calculate_position_score(&pt) - 37.6).abs() > 0.0001);
    /// ```
    pub fn calculate_position_score(&self, patient: &Patient) -> f64 {
        patient.severity_score as f64 * SEVERITY_RATIO
            + patient.time_waited as f64 * TIME_WAITED_RATIO
            + patient.time_to_treat as f64 * SHORTEST_JOB_RATIO
    }
}

#[cfg(test)]
mod tests {
    use super::SchedulerSingleQueue;

    use crate::patient::Patient;

    #[test]
    fn test_calculate_position_score() {
        let pt = Patient {
            id: 1,
            severity_score: 50,
            time_of_arrival: 0,
            time_waited: 30,
            time_to_treat: 18,
        };
        let queue = SchedulerSingleQueue::new();
        let val = queue.calculate_position_score(&pt);
        let desired_ans = 37.6;
        assert!((val - desired_ans).abs() < 0.00001);
    }
}
