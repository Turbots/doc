pub mod diagnosis {
    pub struct Diagnosis {
    }

    impl Diagnosis {
        pub fn new() -> Diagnosis {
            Diagnosis{}
        }
        pub fn perform(&self) {
            println!("Performing Diagnosis...");
        }
    }
}