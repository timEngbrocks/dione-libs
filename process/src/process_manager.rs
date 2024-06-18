use std::{ffi::CString, mem, ptr::null_mut};

use winapi::um::{processthreadsapi::{CreateProcessA, GetExitCodeProcess, PROCESS_INFORMATION, STARTUPINFOA}, synchapi::WaitForSingleObject, winbase::INFINITE};

use crate::util::p_println;

pub struct Process {
	process_information: PROCESS_INFORMATION,
}

impl Process {
	pub fn spawn(binary_path: &str) -> Process {
		let mut si: STARTUPINFOA = unsafe { mem::zeroed() };
    	let mut pi: PROCESS_INFORMATION = unsafe { mem::zeroed() };
		unsafe {
			let cmd = CString::new(format!(
				"{} {}",
				binary_path,
				"child",
			)).expect("Failed to create cstr");
			let success = CreateProcessA(
				null_mut(),
				cmd.as_ptr() as *mut i8,
				null_mut(),
				null_mut(),
				0,
				0,
				null_mut(),
				null_mut(),
				&mut si,
				&mut pi,
			);
			if success == 0 {
				panic!("Failed to create child process!");
			}

			p_println(&format!("Created child process with pid: {}", pi.dwProcessId));
		}

		Process {
			process_information: pi,
		}
	}

	pub fn id(&self) -> u32 {
		self.process_information.dwProcessId
	}

	pub fn join(&self, wait_ms: Option<u32>) {
		unsafe {
			WaitForSingleObject(self.process_information.hProcess, wait_ms.unwrap_or(INFINITE));
		}
	}

	pub fn exit_code(&self) -> u32 {
		unsafe {
			let mut exit_code: u32 = 0;
			GetExitCodeProcess(self.process_information.hProcess, &mut exit_code);
			exit_code
		}
	}
}