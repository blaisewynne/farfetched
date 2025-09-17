use std::process::{Command, Stdio};
use std::process;
use std::env;

pub fn get_terminal() {
   let pid = process::id();
   let term_command = Command::new("cat")
       .arg(format!("/proc/{}/status", pid))
       .output()
       .expect("");
   let terminal = String::from_utf8_lossy(&term_command.stdout);
   print!("{}", terminal.to_string());
}