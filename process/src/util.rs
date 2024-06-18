use winapi::um::processthreadsapi::GetCurrentProcessId;

pub fn p_println(content: &str) {
    let pid = unsafe { GetCurrentProcessId() };
    println!("[{}]: {}", pid, content);
}