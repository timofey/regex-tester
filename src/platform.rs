#[allow(dead_code)]

#[cfg(target_os = "linux")]
pub static EOL: &'static str = "\n";
#[cfg(target_os = "linux")]
pub const EOL_LEN: i8 = 1;

#[cfg(target_os = "macos")]
pub static EOL: &'static str = "\r";
#[cfg(target_os = "macos")]
pub const EOL_LEN: i8 = 1;

#[cfg(target_os = "windows")]
pub static EOL: &'static str = "\r\n";
#[cfg(target_os = "windows")]
pub const EOL_LEN: i8 = 2;

