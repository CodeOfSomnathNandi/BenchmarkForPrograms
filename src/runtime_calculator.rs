use std::mem::size_of;
use std::process::Command;
use std::time::Instant;
use windows::core::imp::HANDLE;
use windows::core::Type;
use windows::Win32::Foundation::FALSE;
use windows::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_CREATE_PROCESS, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

/// Runtime parameter will give in seconds
pub struct RuntimeBound {
    program_name : String,
    runtime: f64
}


impl RuntimeBound {
    pub fn new(program_name :&str) -> Self {
        Self {
            runtime: 0.0,
            program_name: program_name.to_string()
        }
    }

    pub fn execute(&mut self) {
        let mut cmd = Command::new(self.program_name.clone()).spawn().unwrap();
        let start = Instant::now();
        cmd.wait().unwrap();
        let end = Instant::now();
        self.runtime = (end-start).as_secs_f64();
        unsafe {
            let mut process_memory  = PROCESS_MEMORY_COUNTERS::default();
            let process  = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, cmd.id()).expect("Unable to create");
            GetProcessMemoryInfo(process, &mut process_memory, size_of::<PROCESS_MEMORY_COUNTERS>() as u32).expect("Unable to create");
            // 0.000001
            println!("Memory usage: {:?} ", process_memory);
            let total_usage = (process_memory.PeakPagefileUsage + process_memory.PeakWorkingSetSize + process_memory.QuotaPeakPagedPoolUsage + process_memory.QuotaPeakNonPagedPoolUsage) as f64 * 0.000001;
            println!("Memory usage: {:?} mb", total_usage);
        }
    }

}
