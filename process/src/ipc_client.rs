use std::{ffi::CString, io::{BufReader, BufWriter, Error, ErrorKind, Read, Write}, ptr::null_mut};

use winapi::{ctypes::c_void, um::{fileapi::{CreateFileA, ReadFile, WriteFile, OPEN_EXISTING}, handleapi::{CloseHandle, INVALID_HANDLE_VALUE}, namedpipeapi::ConnectNamedPipe, winbase::{CreateNamedPipeA, FILE_FLAG_FIRST_PIPE_INSTANCE, PIPE_ACCESS_DUPLEX, PIPE_TYPE_BYTE, PIPE_UNLIMITED_INSTANCES}, winnt::{FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_ALL}}};

pub const PIPE_NAME: &str = r"\\.\pipe\process-manager-np";

pub struct IPCClient {
	pipe: *mut c_void,
	reader: BufReader<IPCClientReader>,
	writer: BufWriter<IPCClientWriter>,
}

impl IPCClient {
	pub fn construct(pipe_name: &str, create_pipe: bool) -> IPCClient {
		let pipe = if create_pipe {
			IPCClient::create_pipe(pipe_name)
		} else {
			IPCClient::open_pipe(pipe_name)
		};

		IPCClient {
			pipe,
			reader: BufReader::new(IPCClientReader::construct(pipe)),
			writer: BufWriter::new(IPCClientWriter::construct(pipe)),
		}
	}

	pub fn check_for_new_connections(&self) {
		unsafe {
			let success = ConnectNamedPipe(self.pipe, null_mut());
			if success == 0 {
				panic!("Failed to open connection!");
			}
		}
	}

	pub fn buf_reader(&mut self) -> &mut BufReader<IPCClientReader> {
		&mut self.reader
	}

	pub fn buf_writer(&mut self) -> &mut BufWriter<IPCClientWriter> {
		&mut self.writer
	}

	pub fn close(&self) {
		unsafe {
			if CloseHandle(self.pipe) == 0 {
				panic!("Failed to close pipe!");
			}
		}
	}

	fn create_pipe(pipe_name: &str) -> *mut c_void {
		unsafe {
			let pipe_name = CString::new(pipe_name).expect("Failed to create cstr");
			let handle = CreateNamedPipeA(
				pipe_name.as_ptr() as *mut i8,
				PIPE_ACCESS_DUPLEX | FILE_FLAG_FIRST_PIPE_INSTANCE,
				PIPE_TYPE_BYTE,
				PIPE_UNLIMITED_INSTANCES,
				1024,
				1024,
				100,
				null_mut(),
			);
			if handle.is_null() || handle == INVALID_HANDLE_VALUE {
				panic!("Failed to create pipe!");
			}
			handle
		}
	}

	fn open_pipe(pipe_name: &str) -> *mut c_void {
		unsafe {
			let pipe_name = CString::new(pipe_name).expect("Failed to create cstr");
			let handle = CreateFileA(
				pipe_name.as_ptr() as *mut i8,
				GENERIC_ALL,
				FILE_SHARE_READ | FILE_SHARE_WRITE,
				null_mut(),
				OPEN_EXISTING,
				FILE_ATTRIBUTE_NORMAL,
				null_mut(),
			);
			if handle.is_null() || handle == INVALID_HANDLE_VALUE {
				panic!("Failed to open pipe!");
			}
			handle
		}
	}
}

pub struct IPCClientReader {
	pipe: *mut c_void
}

impl IPCClientReader {
	pub fn construct(pipe: *mut c_void) -> IPCClientReader {
		IPCClientReader {
			pipe
		}
	}

	pub fn read_from_pipe(&self, buffer: &mut [u8]) -> std::io::Result<usize> {
		unsafe {
			let mut bytes_read = 0_u32;
			let success = ReadFile(
				self.pipe,
				buffer.as_mut_ptr() as *mut c_void,
				buffer.len() as u32,
				&mut bytes_read as *mut u32,
				null_mut(),
			);
			if success == 0 {
				return Err(Error::new(ErrorKind::Other, "Failed to read from pipe!"));
			}
			Ok(bytes_read as usize)
		}
	}
}

impl Read for IPCClientReader {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		self.read_from_pipe(buf)
	}
}

pub struct IPCClientWriter {
	pipe: *mut c_void
}

impl IPCClientWriter {
	pub fn construct(pipe: *mut c_void) -> IPCClientWriter {
		IPCClientWriter {
			pipe
		}
	}

	pub fn write_to_pipe(&self, buffer: &[u8]) -> std::io::Result<usize> {
		unsafe {
			let mut bytes_written = 0_u32;
			let success = WriteFile(
				self.pipe,
				buffer.as_ptr() as *const c_void,
				buffer.len() as u32,
				&mut bytes_written as *mut u32,
				null_mut(),
			);
			if success == 0 {
				return Err(Error::new(ErrorKind::Other, "Failed to write to pipe!"));
			}
			Ok(bytes_written as usize)
		}
	}
}

impl Write for IPCClientWriter {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.write_to_pipe(buf)
	}

	fn flush(&mut self) -> std::io::Result<()> {
		todo!()
	}
}