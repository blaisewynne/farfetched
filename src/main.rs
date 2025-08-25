use std::process::{Command};
mod linux;
mod mac;

fn main() {
    let cos = get_cos();
    match cos.as_str() {
        "Linux" => linux::main(),
        "Darwin" => mac::main(),
        &_ => println!("error"),
    }
}
fn get_cos() -> String {
   let output = Command::new("uname")
       .output()
       .expect("");
   let cos = String::from_utf8_lossy(&output.stdout);
   cos.to_string()
}