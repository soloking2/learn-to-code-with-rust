struct DentistAppointment {
    doctor: String,
}

impl DentistAppointment {
    fn book<'a>(&self, check_in_time: &'a str, check_out_time: &str) -> &'a str {
        println!(
            "You are booked from {} to {} with doctor {}",
            check_in_time, check_out_time, self.doctor
        );
        check_in_time
    }
}

fn main() {
    let appt = DentistAppointment {
        doctor: String::from("David"),
    };
    let result = appt.book("03:00PM", "11:00AM");
    drop(appt);
    //The result variable is still valid because the check_in_time string literal has a 'static lifetime, 
    //which means it lives for the entire duration of the program. Therefore, even after dropping the appt variable, the result variable can still be used without any issues.  
    println!("{result}");
}
