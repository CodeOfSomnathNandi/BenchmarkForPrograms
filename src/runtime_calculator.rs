use std::process::Command;
use std::time::Instant;

pub struct RuntimeBound {
    program_name : String,
    runtime: usize
}


impl RuntimeBound {
    pub fn new(program_name :&str) -> Self {
        Self {
            runtime: 0,
            program_name: program_name.to_string()
        }
    }

    pub fn execute(&mut self) {
        let mut cmd = Command::new(self.program_name.clone())
            .spawn()
            .unwrap();
        let start = Instant::now();
        cmd.wait().unwrap();
        let end = Instant::now();
        println!("{} secs", (end-start).as_secs_f64());
    }

}
