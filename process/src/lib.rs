#[cfg(target_arch="x86_64")]
#[cfg(target_vendor="pc")]
#[cfg(target_os="windows")]
#[cfg(target_env="msvc")]
pub mod process_manager;

#[cfg(target_arch="x86_64")]
#[cfg(target_vendor="pc")]
#[cfg(target_os="windows")]
#[cfg(target_env="msvc")]
pub mod ipc_client;

#[cfg(target_arch="x86_64")]
#[cfg(target_vendor="pc")]
#[cfg(target_os="windows")]
#[cfg(target_env="msvc")]
pub mod util;