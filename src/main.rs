use doctor::DoctorFactory;

mod doctor;

fn main() {
    let vec_docs = DoctorFactory::generate_vec_doctors(10);
    dbg!(vec_docs);
}
