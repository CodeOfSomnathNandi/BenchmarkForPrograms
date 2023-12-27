use crate::runtime_calculator::RuntimeBound;

mod runtime_calculator;

fn main() {
    let mut program = RuntimeBound::new("powershell.exe");
    program.execute();
}
