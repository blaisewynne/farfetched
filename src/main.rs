use std::process::{Command};
mod linux;
mod mac;

fn main() {
    let cos = get_cos();
    match cos.as_str() {
        "Linux\n" => linux::main(),
        "Darwin\n" => mac::main(),
        _ => println!("Can't find OS."),
    }
}
fn get_cos() -> String {
   let output = Command::new("uname")
       .output()
       .expect("Failed to get OS");
   let cos = String::from_utf8_lossy(&output.stdout);
   cos.to_string()
}