#[derive(Debug)]
struct Patient {
    severity_score: u64,
    time_of_arrival: u64,
    time_waited: u64,
    time_to_treat: u64,
}

struct PatientFactory;

impl PatientFactory {
    fn create_patient(time_of_arrival: u64);
}
