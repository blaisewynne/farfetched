use std::process::{Command, Stdio};

pub fn main() {
  get_user_hostname();
  get_os();
  get_kernel();
  get_terminal();
	get_locale();
  get_ip();
	get_colours();
}

fn get_os() {
   let uname = Command::new("uname")
			.arg("-v")
			.output()
			.expect("");
   let os = String::from_utf8_lossy(&uname.stdout);
   print!("\n{}", os.to_string());
}

fn get_user_hostname() {
   let whoami = Command::new("whoami")
       .output()
       .expect("");
   let user = String::from_utf8_lossy(&whoami.stdout);

   let hostcommand = Command::new("hostname")
       .output()
       .expect("");
   let hostname = String::from_utf8_lossy(&hostcommand.stdout);
   let str_output = format!("\x1b[0;32m{}\x1b[0;37m@\x1b[0;32m{}\x1b[0m\n", user.trim_end(), hostname.trim_end());
   print!("{}", str_output);
   let iter_output = format!("{}@{}", user.trim_end(), hostname.trim_end());
   for _ in 0..iter_output.len() {
       print!("-")
   }
}

fn get_terminal() {
   let terminal_command = Command::new("echo")
       .arg("$SHELL")
       .output()
       .expect("");
   let output = String::from_utf8_lossy(&terminal_command.stdout);
   print!("{}", output.to_string());

}

fn get_kernel() {
   let system_command = Command::new("system_profiler")
       .arg("SPSoftwareDataType")
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let system_sed = Command::new("sed")
       .args(["-n", "s/^.*Kernel Version://p"])
       .stdin(Stdio::from(system_command.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let output = system_sed.wait_with_output().unwrap();
   let kernel = String::from_utf8_lossy(&output.stdout);
   print!("{}", kernel.to_string().trim_start());
}

fn get_locale() {
   let locale_command = Command::new("locale")
	     .stdout(Stdio::piped())
	     .spawn()
	     .unwrap();
   let locale_sed = Command::new("sed")
	     .args(["-n", "s/^.*LC_MESSAGES=//p"])
	     .stdin(Stdio::from(locale_command.stdout.unwrap()))
	     .stdout(Stdio::piped())
       .spawn()
	     .unwrap();
   let locale_tr = Command::new("tr")
       .args(["-d", "\""])
       .stdin(Stdio::from(locale_sed.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let output = locale_tr.wait_with_output().unwrap();
   let locale = String::from_utf8_lossy(&output.stdout);
   print!("{}", locale.to_string());
}

fn get_memory() {
   

}

fn get_ip() {
   let ipconfig = Command::new("ipconfig")
       .args(["getifaddr", "en0"])
       .output()
       .expect("");
   let ip = String::from_utf8_lossy(&ipconfig.stdout);
   print!("{}", ip.to_string());
}

fn get_colours() {
    print!("\x1b[0;30m████\x1b[0;31m████\x1b[0;32m████\x1b[0;33m████\x1b[0;34m████\x1b[0;35m████\x1b[0;36m████\x1b[0;37m████\n");
    print!("\x1b[0;90m████\x1b[0;91m████\x1b[0;92m████\x1b[0;93m████\x1b[0;94m████\x1b[0;95m████\x1b[0;96m████\x1b[0;97m████\n");
}

