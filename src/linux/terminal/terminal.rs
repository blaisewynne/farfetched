use std::process::{Command, Stdio};
use std::process;
use std::env;

pub fn get_terminal() {
   let terminal_environment = "TERM";
   match env::var(terminal_environment).expect("").as_str() {
      "xterm-kitty" => get_kitty(),
      "alacritty" => get_alacritty(),
      _ => print!("Unknown Terminal\n")
   }
}

fn get_kitty() {
   let kitty_version = Command::new("kitty")
       .arg("--version")
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let kitty_grep = Command::new("grep")
       .args(["-oP", "^[^c]*"])
       .stdin(Stdio::from(kitty_version.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let kitty_output = kitty_grep.wait_with_output().unwrap();
   let kitty = String::from_utf8_lossy(&kitty_output.stdout);
   print!("Terminal: {}", kitty.to_string());
}

fn get_alacritty() {
   let alacritty_version = Command::new("alacritty")
       .arg("--version")
       .output()
       .expect("");
   let alacritty = String::from_utf8_lossy(&alacritty_version.stdout);
   print!("Terminal: {}", alacritty.to_string());



}