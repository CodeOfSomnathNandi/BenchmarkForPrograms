use crate::runtime_calculator::RuntimeBound;

mod runtime_calculator;

fn main() {
    let mut program = RuntimeBound::new("notepad.exe");
    program.execute();
}
