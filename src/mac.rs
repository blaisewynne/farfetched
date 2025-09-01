use std::process::{Command, Stdio};

pub fn main() {
  get_user_hostname();
  get_os();
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
   print!("{}", os.to_string());
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
   print!("{}", str_output)
   :w
   for _ in 0..str_output.len() / 2 {
       print!("-")

   }
}

fn get_locale() {
   let locale_command = Command::new("locale")
	     .stdout(Stdio::piped())
	     .spawn()
	     .unwrap();
   let locale_grep = Command::new("ggrep")
	     .args(["-oP", r"LC_ALL=\K.*"])
	     .stdin(Stdio::from(locale_command.stdout.unwrap()))
	     .stdout(Stdio::piped())
       .spawn()
	     .unwrap();
   let locale_tr = Command::new("tr")
       .args(["-d", "\""])
       .stdin(Stdio::from(locale_grep.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let output = locale_tr.wait_with_output().unwrap();
   let locale = String::from_utf8_lossy(&output.stdout);
   print!("{}", locale.to_string());
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

