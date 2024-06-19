use std::io::{BufRead, Write};
use std::{env, process::exit};

use process::ipc_client::{IPCClient, PIPE_NAME};
use process::process_manager::Process;
use process::util::p_println;

#[cfg(target_arch="x86_64")]
#[cfg(target_vendor="pc")]
#[cfg(target_os="windows")]
#[cfg(target_env="msvc")]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("root")) {
        root_process();
    } else if args.contains(&String::from("child")) {
        assert!(args.len() == 2);
        child_process();
    }
}

pub fn root_process() {
    p_println("Root process running. Spawning child.");

    let binary_path = match env::var("BINARY_PATH") {
        Ok(v) => v,
        Err(e) => panic!("BINARY_PATH has not been set! Got: '{e}'"),
    };
    let process = Process::spawn(&binary_path);
    
    let mut ipc_client = IPCClient::construct(PIPE_NAME, true);
    ipc_client.check_for_new_connections();
    
    let buf_writer = ipc_client.buf_writer();
    match buf_writer.write_all(b"Hello from parent via IPC!\n") {
        Ok(_) => (),
        Err(e) => panic!("IPCClient buf_writer failed. Got: '{e}'"),
    };
    match buf_writer.flush() {
        Ok(_) => (),
        Err(e) => panic!("IPCClient buf_writer failed to flush. Got: '{e}'"),
    };

    let buf_reader = ipc_client.buf_reader();
    let mut buffer = String::new();
    match buf_reader.read_line(&mut buffer) {
        Ok(_) => (),
        Err(e) => panic!("IPCClient buf_reader failed. Got: '{e}'")
    };
    p_println(&format!("Received message from child: '{}'", buffer.strip_suffix("\n").unwrap()));

    ipc_client.close();
    
    process.join(None);
    p_println(&format!("Child exited with code: {}", process.exit_code()));

    exit(0);
}

pub fn child_process() {
    p_println("Child process started.");

    let mut ipc_client = IPCClient::construct(PIPE_NAME, false);

    let buf_reader = ipc_client.buf_reader();
    let mut buffer = String::new();
    match buf_reader.read_line(&mut buffer) {
        Ok(_) => (),
        Err(e) => panic!("IPCClient buf_reader failed. Got: '{e}'")
    };
    p_println(&format!("Received message from parent: '{}'", buffer.strip_suffix("\n").unwrap()));

    let buf_writer = ipc_client.buf_writer();
    match buf_writer.write_all(b"Thank you from child!\n") {
        Ok(_) => (),
        Err(e) => panic!("IPCClient buf_writer failed. Got: '{e}'"),
    };
    match buf_writer.flush() {
        Ok(_) => (),
        Err(e) => panic!("IPCClient buf_writer failed to flush. Got: '{e}'"),
    };

    ipc_client.close();

    p_println("Child process is finished. Exiting.");
    exit(0);
}
